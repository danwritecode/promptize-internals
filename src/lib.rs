pub trait Promptize<T>: Send {
    fn reassemble(
        &self, 
        model: &str, 
        token_limit: i32,
        maximum_chunk_count: i32,
        chunk_summary: String, 
    ) -> anyhow::Result<(std::vec::Vec<std::vec::Vec<tiktoken_rs::ChatCompletionRequestMessage>>, T)>;

    fn build_prompt(
        self, 
        model: &str, 
        token_limit: i32,
        maximum_chunk_count: i32
    ) -> anyhow::Result<(std::vec::Vec<std::vec::Vec<tiktoken_rs::ChatCompletionRequestMessage>>, T)>;
}
