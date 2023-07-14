pub trait Promptize<T>: Send {
    fn reassemble(
        &self, 
        chunk_summary: String, 
    ) -> anyhow::Result<(std::vec::Vec<std::vec::Vec<tiktoken_rs::ChatCompletionRequestMessage>>, T)>;

    fn build_prompt(
        self
    ) -> anyhow::Result<(std::vec::Vec<std::vec::Vec<tiktoken_rs::ChatCompletionRequestMessage>>, T)>;

    fn get_model(&self) -> Option<String>;
}

macro_rules! starts_with_any {
    ($str:expr, $($prefix:expr),* $(,)?) => {
        false $(|| $str.starts_with($prefix))*
    };
}

pub fn get_context_size(model: &str) -> i32 {
    if starts_with_any!(model, "gpt-4-32k") {
        return 32768;
    }
    if starts_with_any!(model, "gpt-4") {
        return 8192;
    }
    if starts_with_any!(model, "gpt-3.5-turbo") {
        return 4096;
    }
    if starts_with_any!(model, "text-davinci-002", "text-davinci-003") {
        return 4097;
    }
    if starts_with_any!(model, "ada", "babbage", "curie") {
        return 2049;
    }
    if starts_with_any!(model, "code-cushman-001") {
        return 2048;
    }
    if starts_with_any!(model, "code-davinci-002") {
        return 8001;
    }
    if starts_with_any!(model, "davinci") {
        return 2049;
    }
    if starts_with_any!(model, "text-ada-001", "text-babbage-001", "text-curie-001") {
        return 2049;
    }
    4096
}
