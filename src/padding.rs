pub fn pad(buf: &mut Vec<u8>, block_size: usize) {
    buf.push(0x80);
    let overflow = buf.len() % block_size;
    let mut zeroes = vec![0; (block_size - overflow) % block_size];
    buf.append(&mut zeroes);
}

pub fn unpad(buf: &mut Vec<u8>) {
    let mut i = buf.len();
    loop {
        i -= 1;
        if buf[i] == 0x80 {
            buf.truncate(i);
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pad_one_byte() {
        let mut buf = vec![1, 2, 3];
        pad(&mut buf, 4);

        let expected = vec![1, 2, 3, 0x80];
        assert_eq!(buf, expected);
    }

    #[test]
    fn pad_multiple_bytes() {
        let mut buf = vec![1; 4];
        pad(&mut buf, 8);

        let expected = vec![1, 1, 1, 1, 0x80, 0, 0, 0];
        assert_eq!(buf, expected);
    }

    #[test]
    fn pad_overflow() {
        let mut buf = vec![1; 3];
        pad(&mut buf, 3);

        let expected = vec![1, 1, 1, 0x80, 0, 0];
        assert_eq!(buf, expected);
    }

    #[test]
    fn unpad_one() {
        let mut buf = vec![1, 1, 1, 0x80];
        unpad(&mut buf);

        assert_eq!(buf, vec![1; 3]);
    }

    #[test]
    fn unpad_many() {
        let mut buf = vec![1, 1, 1, 1, 0x80, 0, 0];
        unpad(&mut buf);

        assert_eq!(buf, vec![1; 4]);
    }
}
