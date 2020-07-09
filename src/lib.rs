use vtext::tokenize::Tokenizer;
use vtext::tokenize_sentence::*;
use std::path::PathBuf;
use subprocess::Exec;

pub struct Pool{
    metamap_pool: Vec<Metamap>,
}

pub struct MMIConcepts {
    index: String,
    mm: String,
    score: String, 
    preferred_name: String, 
    cui: String, 
    semtypes: String,
    trigger: String, 
    location: String, 
    pos_info: String, 
    tree_codes: String,
}

impl MMIConcepts {
    fn from<'a> (line: String) -> MMIConcepts {
        let args: Vec<&str> = line.split("|").collect();

        MMIConcepts{
            index: args[0].to_string(),
            mm: args[1].to_string(),
            score: args[2].to_string(), 
            preferred_name: args[3].to_string(), 
            cui: args[4].to_string(), 
            semtypes: args[5].to_string(),
            trigger: args[6].to_string(), 
            location: args[7].to_string(), 
            pos_info: args[8].to_string(), 
            tree_codes: args[9].to_string(),
        }
    }
}

pub struct Metamap {
    segmenter: Box<dyn Tokenizer>,
    cmd: Vec<String>,
    fatal_flag: bool,
}

impl Metamap {
    pub fn new(filename: &PathBuf) -> Metamap {
        let mut args = Vec::new();
        args.push(filename.clone().into_os_string().into_string().unwrap());
        args.push("-N".to_string());

        Metamap {
            segmenter: Box::new(UnicodeSentenceTokenizer::default()),
            cmd: args,
            fatal_flag: false,
        }
    }

    pub fn extract_concepts(&mut self, input: &str, segment_sentence: bool) -> Vec<&str> { 
        let mmCommand = self.cmd.join(" ");

        let sentences = match segment_sentence {
            true => self.segmenter.tokenize(input).collect(),
            false => vec![input],
        }.join("\n");

        let raw_output = {Exec::shell(format!("echo -e {}", sentences)) | Exec::cmd(self.cmd.join(" "))}
                        .capture()
                        .unwrap();

        vec![]
    } 

    pub fn composite_phrase<'a>(&'a mut self, num_phrases: usize) -> &'a Metamap {
        self.cmd.push(format!("-Q {}", num_phrases));
        self
    }

    pub fn add_word_sense_disambiguation<'a> (&'a mut self) -> &'a mut Metamap {
        self.cmd.push("-y".to_string());
        self
    }

    pub fn strict_model<'a> (&'a mut self) -> &'a mut Metamap {
        self.cmd.push("-A".to_string());
        self
    }

    pub fn relaxed_model<'a> (&'a mut self) -> &'a mut Metamap {
        self.cmd.push("-C".to_string());
        self
    }

    pub fn allow_large_n<'a> (&'a mut self) -> &'a mut Metamap {
        self.cmd.push("-l".to_string());
        self
    }

    pub fn allow_overmatches<'a> (&'a mut self) -> &'a mut Metamap {
        self.cmd.push("-o".to_string());
        self
    }

    pub fn allow_concept_gaps<'a> (&'a mut self) -> &'a mut Metamap {
        self.cmd.push("-g".to_string());
        self
    }

    pub fn term_processing<'a> (&'a mut self) -> &'a mut Metamap {
        self.cmd.push("-z".to_string());
        self
    }

    pub fn no_derivational_variants<'a> (&'a mut self) -> &'a mut Metamap {
        self.cmd.push("-d".to_string());
        self
    }

    pub fn ignore_word_order<'a> (&'a mut self) -> &'a mut Metamap {
        self.cmd.push("-i".to_string());
        self
    }

    pub fn allow_acronym_variants<'a> (&'a mut self) -> &'a mut Metamap {
        self.cmd.push("-a".to_string());
        self.fatal_flag = match self.fatal_flag {
            false => true,
            true => panic!("Cannot have both allow_acronym_variants and unique acronym variants"),
        };
        self
    }

    pub fn unique_acronym_variants<'a> (&'a mut self) -> &'a mut Metamap {
        self.cmd.push("-u".to_string());
        self.fatal_flag = match self.fatal_flag {
            false => true,
            true => panic!("Cannot have both allow_acronym_variants and unique acronym variants"),
        };
        self
    }

    pub fn prefer_multiple_concepts<'a> (&'a mut self) -> &'a mut Metamap {
        self.cmd.push("-Y".to_string());
        self
    }

    pub fn ignore_stop_phrases<'a> (&'a mut self) -> &'a mut Metamap {
        self.cmd.push("-K".to_string());
        self
    }

    pub fn compute_all_mappings<'a> (&'a mut self) -> &'a mut Metamap {
        self.cmd.push("-b".to_string());
        self
    }

    pub fn exclude_sources<'a> (&'a mut self, sources: Vec<String>) -> &'a mut Metamap {
        self.cmd.push("-e".to_string());
        self.cmd.push(sources.join(","));
        self
    }

    pub fn restrict_to_sources<'a> (&'a mut self, sources: Vec<String>) -> &'a mut Metamap {
        self.cmd.push("-R".to_string());
        self.cmd.push(sources.join(","));
        self
    }
    pub fn restrict_to_sts<'a> (&'a mut self, sts: Vec<String>) -> &'a mut Metamap {
        self.cmd.push("-J".to_string());
        self.cmd.push(sts.join(","));
        self
    }
    pub fn exclude_sts<'a> (&'a mut self, sources: Vec<String>) -> &'a mut Metamap {
        self.cmd.push("-k".to_string());
        self.cmd.push(sources.join(","));
        self
    }

    pub fn no_nums<'a> (&'a mut self, nums: Vec<String>) -> &'a mut Metamap {
        self.cmd.push("--no_nums".to_string());
        self.cmd.push(nums.join(","));
        self
    }
}