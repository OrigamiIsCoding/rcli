mod test {

    #[test]
    fn test_word_count() {
        let contents = "hello world hello world";
        let counter = crate::word_count(contents);
        assert_eq!(counter.get("hello"), Some(&2));
        assert_eq!(counter.get("world"), Some(&2));
        
        let contents = "hello world\n             hello\n world";
        let counter = crate::word_count(contents);
        assert_eq!(counter.get("hello"), Some(&2));
        assert_eq!(counter.get("world"), Some(&2));
    }
}