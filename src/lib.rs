use vtext::tokenize::Tokenizer;
use vtext::tokenize_sentence::*;
use std::path::PathBuf;
use subprocess::Exec;

pub struct Pool{
    metamap_pool: Vec<Metamap>,
}

#[derive(Debug)]
pub struct MetamapLiteMMIConcepts {
    index: String, 
    mm: String, 
    score: String, 
    preferred_name: String, 
    cui: String, 
    semtypes: String, 
    trigger: String, 
    pos_info: String, 
    tree_codes: String
}

impl MetamapLiteMMIConcepts {
    fn from<'a> (line: String) -> MetamapLiteMMIConcepts {
        let args: Vec<&str> = line.split("|").collect();

        MetamapLiteMMIConcepts{
            index: args[0].to_string(), 
            mm: args[1].to_string(), 
            score: args[2].to_string(), 
            preferred_name: args[3].to_string(), 
            cui: args[4].to_string(), 
            semtypes: args[5].to_string(), 
            trigger: args[6].to_string(), 
            pos_info: args[7].to_string(), 
            tree_codes: args[8].to_string()
        }
    }
}

#[derive(Debug)]
pub struct MetamapMMIConcepts {
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

impl MetamapMMIConcepts {
    fn from<'a> (line: String) -> MetamapMMIConcepts {
        let args: Vec<&str> = line.split("|").collect();

        MetamapMMIConcepts{
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

pub struct MetamapLite {
    //segmenter: Box<dyn Tokenizer>,
    cmd: Vec<String>,
}

impl MetamapLite {
    pub fn new(filename: &PathBuf) -> MetamapLite {
        let mut args = Vec::new();
        args.push(filename.clone().into_os_string().into_string().unwrap());
        args.push("-N".to_string());

        MetamapLite {
            // segmenter: Box::new(UnicodeSentenceTokenizer::default()),
            cmd: args,
        }
    }

    pub fn extract_concepts(&self, input: &str, segment_sentence: bool) -> Vec<MetamapMMIConcepts> { 
        let mut mm_command = self.cmd.join(" ");
        mm_command.push_str("--pipe");

        let sentences = match segment_sentence {
            //true => self.segmenter.tokenize(input).collect(),
            true => vec![],
            false => vec![input],
        }.join("\n");

        let shell_command = format!("echo {}", sentences);

        {Exec::shell(shell_command) | Exec::shell(mm_command)}
                        .capture()
                        .unwrap()
                        .stdout_str()
                        .lines()
                        .skip(1)
                        .map(|e| MetamapLiteMMIConcepts::from(e.into()))
                        .collect()
    } 

    pub fn restrict_to_sources<'a> (&'a mut self, sources: Vec<String>) -> &'a mut MetamapLite {
        self.cmd.push("--restrict_to_sources".to_string());
        self.cmd.push(sources.join(","));
        self
    }

    pub fn restrict_to_sts<'a> (&'a mut self, sts: Vec<String>) -> &'a mut MetamapLite {
        self.cmd.push("--restrict_to_sts".to_string());
        self.cmd.push(sts.join(","));
        self
    }
}

pub struct Metamap {
    //segmenter: Box<dyn Tokenizer>,
    cmd: Vec<String>,
    fatal_flag: bool,
}


impl Metamap {
    pub fn new(filename: &PathBuf) -> Metamap {
        let mut args = Vec::new();
        args.push(filename.clone().into_os_string().into_string().unwrap());
        args.push("-N".to_string());

        Metamap {
            // segmenter: Box::new(UnicodeSentenceTokenizer::default()),
            cmd: args,
            fatal_flag: false,
        }
    }

    pub fn extract_concepts(&self, input: &str, segment_sentence: bool) -> Vec<MetamapMMIConcepts> { 
        let mut mm_command = self.cmd.join(" ");
        mm_command.push_str(" --silent");

        let sentences = match segment_sentence {
            //true => self.segmenter.tokenize(input).collect(),
            true => vec![],
            false => vec![input],
        }.join("\n");

        let shell_command = format!("echo {}", sentences);

        {Exec::shell(shell_command) | Exec::shell(mm_command)}
                        .capture()
                        .unwrap()
                        .stdout_str()
                        .lines()
                        .skip(1)
                        .map(|e| MetamapMMIConcepts::from(e.into()))
                        .collect()
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