pub fn compare(local: u32, online: u32) -> i8 {
    if local == online {
        return 0;
    } else if local < online {
        return 1;
    } else {
        return -1;
    }
}
