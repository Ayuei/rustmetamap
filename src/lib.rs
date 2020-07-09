use vtext::tokenize::{VTextTokenizerParams,Tokenizer};

pub struct Pool{
    metamap_pool: Vec<Metamap>,
}

pub struct Metamap {
    segmenter: Tokenizer,
    
} 


// IF ((right context = period + space + capital letter
//OR period + quote + space + capital letter
//OR period + space + quote + capital letter)
//AND (left context != abbreviation))

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