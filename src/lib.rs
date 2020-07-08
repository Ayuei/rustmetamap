use punkt::{TrainingData, SentenceTokenizer, tokenizer};

pub struct Pool{
    metamap_pool: Vec<Metamap>,
}

pub struct Metamap {
    model_data: TrainingData,
    segmenter: SentenceTokenizer,
} 

impl Metamap {
    pub fn segment_sentence(self, input: &str) -> Box<dyn tokenizer> {
        SentenceTokenizer::<Standard>::new(&str, &self.model_data)
    }

    pub fn extract_concepts(input: &str, segment_sentence: bool) { 
        segment_sentence(&input)
            .iter()
            .map()
    } 
}