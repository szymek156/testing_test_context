fn main() {
    println!("Hello, world!");
}

mod test {
    use test_context::{test_context, AsyncTestContext};

    // use test_context::futures::FutureExt;
    struct TestData {}

    #[async_trait::async_trait]
    impl AsyncTestContext for TestData {
        async fn setup() -> Self {
            Self{}
        }

        async fn teardown(mut self) {}
    }

    #[test_context(TestData)]
    #[tokio::test]
    async fn test_works(ctx: &TestData) {
    }
}
