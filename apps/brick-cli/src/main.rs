use brick_core::Ledger;
use brick_chain::{state_merkle, balance_proof, verify_balance};
use brick_lanes::pack_lanes;
use brick_holo::{HoloNode, ZoneConfig};
use brick_hash::hex16;
use brick_ids::account_id_from_name;
use ed25519_dalek::SigningKey;
use std::env;

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() { return help(); }
    match args[0].as_str() {
        "hello" => println!("brick-cli: hello 👋"),
        "mint" => cmd_mint(&args),
        "transfer" => cmd_transfer(&args),
        "lanes" => cmd_lanes(&args),
        "holo_tick" => cmd_holo_tick(),
        "holo_run" => cmd_holo_run(&args),
        "prove" => cmd_prove(&args),
        _ => help(),
    }
}

fn help() {
    eprintln!("brick-cli:
  hello
  mint <to> <amount>
  transfer <from> <to> <amount>
  lanes [N]
  holo_tick
  holo_run [TICKS]
  prove <name>");
}

fn cmd_mint(args: &[String]) {
    if args.len() < 3 { eprintln!("usage: mint <to> <amount>"); return; }
    let to = &args[1]; let amount: u128 = args[2].parse().unwrap_or(0);
    let mut l = Ledger::new();
    let _ = l.mint(to, amount, "mint");
    let (r, _) = state_merkle(&l.balances);
    println!("ok: {} += {}  state_root={}", to, amount, hex16(&r));
}

fn cmd_transfer(args: &[String]) {
    if args.len() < 4 { eprintln!("usage: transfer <from> <to> <amount>"); return; }
    let from = &args[1]; let to = &args[2]; let amount: u128 = args[3].parse().unwrap_or(0);
    let mut l = Ledger::new();
    let _ = l.mint(from, amount, "bootstrap");
    // IMPORTANT: don't keep a &Transaction; just keep a bool so the mutable borrow ends here.
    let ok = l.transfer(from, to, amount, "pay").is_ok();
    let (sr, _) = state_merkle(&l.balances);
    println!("transfer ok? {}  state_root={}", ok, hex16(&sr));
}

fn cmd_lanes(args: &[String]) {
    let n: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(6);
    let mut l = Ledger::new();
    let _ = l.mint("alice", 10_000, "boot");
    for _ in 0..n { let _ = l.transfer("alice","bob",100,"demo"); }
    let lanes = pack_lanes(&l.txs);
    println!("packed {n} tx into {} lane(s)", lanes.len());
    for (i, lane) in lanes.iter().enumerate() {
        let ids: Vec<_> = lane.iter().map(|t| format!("#{}", t.id)).collect();
        println!("  lane {i}: {}", ids.join(" "));
    }
}

fn deterministic_key(node_id: u32) -> SigningKey {
    let mut h = blake3::Hasher::new();
    h.update(b"BRICK/DEVKEY/v1");
    h.update(&node_id.to_le_bytes());
    let bytes = h.finalize();
    let mut seed = [0u8; 32];
    seed.copy_from_slice(bytes.as_bytes());
    SigningKey::from_bytes(&seed)
}

fn cmd_holo_tick() {
    let key = deterministic_key(1);
    let mut node = HoloNode::new(ZoneConfig { node_id: 1, zone_id: 0, batch_max: 8192 }, key);
    let _ = node.ledger.mint("alice", 1_000, "boot");
    let _ = node.ledger.transfer("alice","bob",250,"pay");
    println!("pending before tick: {}", node.pending_len());
    let out = node.tick();
    if let Some(c) = &out.claim {
        println!("claim: zone={} h={} tx={} state={} da={}",
            c.commit.zone, c.commit.height, c.tx_count,
            hex16(&c.commit.state_root), hex16(&c.commit.da_root));
    } else {
        println!("no new claim this tick");
    }
    println!("super_root={}", hex16(&out.super_root));
    println!("att.hash={}", hex16(&out.attestation.hash));
}

fn cmd_holo_run(args: &[String]) {
    let ticks: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(5);
    let key = deterministic_key(1);
    let mut node = HoloNode::new(ZoneConfig { node_id: 1, zone_id: 0, batch_max: 8_192 }, key);
    let _ = node.ledger.mint("alice", 50_000, "boot");
    for t in 0..ticks {
        for _ in 0..100 { let _ = node.ledger.transfer("alice","bob",1,format!("t{t}")); }
        let out = node.tick();
        println!("[epoch {}] pending={} state={} super={}",
            t, node.pending_len(),
            hex16(&out.attestation.header.state_root),
            hex16(&out.super_root));
    }
}

fn cmd_prove(args: &[String]) {
    if args.len() < 2 { eprintln!("usage: prove <name>"); return; }
    let name = &args[1];
    let mut l = Ledger::new();
    let _ = l.mint("alice", 1000, "boot");
    let _ = l.transfer("alice","bob",250,"pay");
    let acct = account_id_from_name(name);
    match balance_proof(&l.balances, acct) {
        Some((bal, proof, idx, root)) => {
            let ok = verify_balance(acct, bal, idx, root, &proof);
            println!("prove({name}): bal={bal} root={} ok={}", hex16(&root), ok);
        }
        None => eprintln!("no such account"),
    }
}
