use idalib::func::FunctionFlags;
use idalib::idb::*;

fn main() -> anyhow::Result<()> {
    let idb = IDB::open("./tests/ls")?;

    println!(
        "proc (short/long): {}/{}",
        idb.processor().short_name(),
        idb.processor().long_name()
    );

    println!("compiler: {:?}", idb.meta().cc_id());
    println!("filetype: {:?}", idb.meta().filetype());
    println!("procname: {}", idb.meta().procname());

    println!("segments: {}", idb.segment_count());

    for (sid, s) in idb.segments() {
        let addr = s.start_address();
        let size = s.len();

        println!("segment {sid} @ {addr:x} ({size} bytes)");

        let name = s.name();
        let bytes = s.bytes();
        let align = s.alignment();
        let bitness = s.bitness();
        let perms = s.permissions();

        println!("- name: {name:?}");
        println!("- bytes: {:x?}", &bytes[..bytes.len().min(16)]);
        println!("- alignment: {:?}", align);
        println!("- bitness: {}", 1 << (bitness + 4));
        println!("- permissions: {:?}", perms);
    }

    println!("functions: {}", idb.function_count());

    for (fid, f) in idb.functions() {
        if f.flags().contains(FunctionFlags::TAIL) {
            continue;
        }

        let addr = f.start_address();
        let fcfg = f.cfg()?;

        println!(
            "function {fid} @ {addr:x}: {:?} (blocks: {})",
            f.name(),
            fcfg.blocks_count()
        );

        for (id, blk) in fcfg.blocks().enumerate() {
            println!(
                "--- blk {id} @ {:x}-{:x} ---",
                blk.start_address(),
                blk.end_address()
            );

            if blk.len() != 0 {
                let insn = idb.insn_at(blk.start_address()).expect("first instruction");
                println!(
                    "- insn: (ea: {:x}, size: {:x}, operands: {}) ",
                    insn.address(),
                    insn.len(),
                    insn.operand_count()
                );
            }

            println!("- ret: {}", blk.is_ret());
            println!("- noret: {}", blk.is_noret());
            println!("- enoret: {}", blk.is_enoret());
            println!("- cndret: {}", blk.is_cndret());
            println!("- indjump: {}", blk.is_indjump());
            println!("- extern: {}", blk.is_extern());

            println!(
                "preds: {}",
                blk.preds_with(&fcfg)
                    .map(|blk| format!("{:x}", blk.start_address()))
                    .collect::<Vec<_>>()
                    .join(",")
            );

            println!(
                "succs: {}",
                blk.succs_with(&fcfg)
                    .map(|blk| format!("{:x}", blk.start_address()))
                    .collect::<Vec<_>>()
                    .join(",")
            );
        }
    }

    Ok(())
}