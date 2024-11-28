use lsm_tree::{AbstractTree, Config, SequenceNumberCounter};
use test_log::test;

#[test]
fn segment_reader_mvcc_slab() -> lsm_tree::Result<()> {
    const ITEM_COUNT: usize = 10_000;

    let folder = tempfile::tempdir()?;

    let tree = Config::new(&folder)
        .data_block_size(1_024)
        .index_block_size(1_024)
        .open()?;

    let seqno = SequenceNumberCounter::default();

    for _ in 0..ITEM_COUNT {
        tree.insert("a", "", seqno.next());
    }
    tree.insert("b", "", 0);

    tree.flush_active_memtable(0)?;

    let level_manifest = tree.levels.read();

    let segment = level_manifest
        .levels
        .first()
        .expect("should exist")
        .segments
        .first()
        .expect("should exist");

    let reader = segment.iter();
    assert_eq!(reader.count(), ITEM_COUNT + 1);

    Ok(())
}

#[test]
fn segment_reader_mvcc_slab_blob() -> lsm_tree::Result<()> {
    const ITEM_COUNT: usize = 1_000;

    let folder = tempfile::tempdir()?;

    let tree = Config::new(&folder)
        .data_block_size(1_024)
        .index_block_size(1_024)
        .open_as_blob_tree()?;

    let seqno = SequenceNumberCounter::default();

    for _ in 0..ITEM_COUNT {
        tree.insert("a", "neptune".repeat(10_000), seqno.next());
    }
    tree.insert("b", "", 0);

    tree.flush_active_memtable(0)?;

    let level_manifest = tree.index.levels.read();

    let segment = level_manifest
        .levels
        .first()
        .expect("should exist")
        .segments
        .first()
        .expect("should exist");

    let reader = segment.iter();
    assert_eq!(reader.count(), ITEM_COUNT + 1);

    Ok(())
}
