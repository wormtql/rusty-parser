use grammar::grammar::Grammar;
// use grammar::grammar::letter::Letter;
// use grammar::grammar::rule::Rule;
use grammar::first_follow_set::FirstFollowSet;
// use grammar::lr0::lr0_item_set::LR0ItemSet;
// use grammar::lr0::lr0_item::LR0Item;
use grammar::lr0::lr0_item_set_family::LR0ItemSetFamily;

use grammar::lr1::lr1_item_set_family::LR1ItemSetFamily;

use grammar::{lr0, slr1, ll1, lr1, lalr1};
use grammar::lr_table::LRTable;

use grammar::parse_tree::ParseTree;
use grammar::token::TokenStream;

use clap::{Arg, App};

use std::fs::File;
use std::fs;
use std::io::prelude::*;

fn main() {
    
    let matches = App::new("Rusty Parser")
        .version("0.1.0")
        .author("wormtql <584130248@qq.com>")
        .arg(Arg::with_name("out")
                .long("out")
                .short("o")
                .takes_value(true)
                .help("specifies output file"))
        .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("input file"))
        .arg(Arg::with_name("lr0")
                .long("lr0")
                .takes_value(false)
                .help("LR(0)"))
        .arg(Arg::with_name("slr1")
                .long("slr1")
                .takes_value(false)
                .help("SLR(0)"))
        .arg(Arg::with_name("lr1")
                .long("lr1")
                .takes_value(false)
                .help("LR(1)"))
        // .arg(Arg::with_name("lalr1")
        //         .long("lalr1")
        //         .takes_value(false)
        //         .help("LALR(1)"))
        // .arg(Arg::with_name("ll1")
        //         .long("ll1")
        //         .takes_value(false)
        //         .help("perform LL(1) analysis"))
        // .arg(Arg::with_name("which")
        //         .long("which")
        //         .short("w")
        //         .takes_value(false)
        //         .help("determine which LR grammar"))
        // .arg(Arg::with_name("expand")
        //         .long("expand")
        //         .takes_value(false)
        //         .help("expand the input grammar"))
        // .arg(Arg::with_name("pg")
        //         .long("pg")
        //         .takes_value(false)
        //         .help("print grammar"))
        .arg(Arg::with_name("cache")
                .long("cache")
                .takes_value(true)
                .help("cache the LR table to output file"))
        .arg(Arg::with_name("cached_lr_table")
                .long("cached-lr-table")
                .takes_value(true)
                .help("using cached LR table"))
        .arg(Arg::with_name("lr_analysis")
                .long("lr-analysis")
                .takes_value(false)
                .help("perform LR analysis"))
        .arg(Arg::with_name("token")
                .long("token")
                .takes_value(true)
                .help("input token file"))
        // .arg(Arg::with_name("is_ll1")
        //         .long("is-ll1")
        //         .takes_value(false)
        //         .help("determine is LL(1) grammar"))
        .get_matches();


    if matches.is_present("cache") {
        let input_file = matches.value_of("cache").unwrap();
        let output_file = matches.value_of("out").unwrap();

        println!("reading grammar");
        let grammar = Grammar::from_advanced_file(&input_file).unwrap();
        

        let table;
        if matches.is_present("lr0") {
            println!("generating LR(0) table");
            table = lr0::lr_table_from_grammar(&grammar).unwrap();
        } else if matches.is_present("slr1") {
            println!("generating SLR(1) table");
            table = slr1::lr_table_from_grammar(&grammar).unwrap();
        } else if matches.is_present("lr1") {
            println!("generating LR(1) table");
            table = lr1::lr_table_from_grammar(&grammar).unwrap();
        } else {
            println!("you have to specify an algorithm, eg, --lr1");
            return;
        }

        println!("writing file");
        let serialized = serde_json::to_string(&table).unwrap();
        let mut file = File::create(&output_file).unwrap();
        file.write_all(serialized.as_bytes());
        println!("done!");

        return;
    }

    // if matches.is_present("slr1") {
    //     let table;
    //     // load table
    //     if matches.is_present("cached_lr_table") {
    //         let f = matches.value_of("cached_lr_table").unwrap();
    //         let content = fs::read_to_string(f).unwrap();

    //         let de: LRTable = serde_json::from_str(&content).unwrap();
    //         table = de;
    //     } else {
    //         let f = matches.value_of("file").unwrap();

    //         let mut grammar = Grammar::from_advanced_file(&f).unwrap();
    //         if matches.is_present("expand") {
    //             grammar.expand();
    //         }

    //         table = match slr1::lr_table_from_grammar(&grammar) {
    //             Ok(t) => t,
    //             Err(s) => panic!("{}", s),
    //         };
    //     }

    //     let token_file = matches.value_of("lr1").unwrap();
        
    //     let token_stream = TokenStream::from_file(&token_file);
    //     println!("{}", token_stream);

    //     let tree = table.analysis(&token_stream);
    // }

    if matches.is_present("lr_analysis") {
        let table;

        let token_file = matches.value_of("token").unwrap();
        let output_file = matches.value_of("out").unwrap();

        // load table
        if matches.is_present("cached_lr_table") {
            let f = matches.value_of("cached_lr_table").unwrap();
            let content = fs::read_to_string(f).unwrap();

            let de: LRTable = serde_json::from_str(&content).unwrap();
            table = de;
        } else {
            let f = matches.value_of("file").unwrap();

            let mut grammar = Grammar::from_advanced_file(&f).unwrap();
            if matches.is_present("lr0") {
                table = lr0::lr_table_from_grammar(&grammar).unwrap();
            } else if matches.is_present("slr1") {
                table = slr1::lr_table_from_grammar(&grammar).unwrap();
            } else if matches.is_present("LR(1)") {
                table = lr1::lr_table_from_grammar(&grammar).unwrap();
            } else {
                println!("you have to specify an algorithm, eg, --lr1");
                return;
            }
        }
        
        let token_stream = TokenStream::from_file(&token_file);
        // println!("{}", token_stream);

        let tree = table.analysis(&token_stream);

        let json = serde_json::to_string(&tree).unwrap();
        let mut outf = File::create(&output_file).unwrap();
        outf.write_all(json.as_bytes());
    }


    // let file = matches.value_of("file").unwrap();
    // println!("the file passed is: {}", file);

    // let mut grammar = Grammar::from_advanced_file(&file).unwrap();
    // if matches.is_present("expand") {
        // grammar.expand();
    // }

    // if matches.is_present("lr0_family") {
    //     let lr0_family = LR0ItemSetFamily::from_grammar(&grammar);
    //     println!("{}", lr0_family);
    // }
    // if matches.is_present("lr1_family") {
    //     let lr1_family = LR1ItemSetFamily::from_grammar(&grammar);
    //     println!("{}", lr1_family);
    // }
    // if matches.is_present("ffset") {
    //     let ffset = FirstFollowSet::from_grammar(&grammar);
    //     println!("{}", ffset);
    // }
    // if matches.is_present("pg") {
    //     println!("{}", grammar);
    // }

    // if matches.is_present("lr0") {
    //     match lr0::lr_table_from_grammar(&grammar) {
    //         Ok(t) => println!("{}", t),
    //         Err(s) => println!("{}", s),
    //     };
    // }
    
    
    // if matches.is_present("lalr1") {
    //     match lalr1::lalr_table_from_grammar(&grammar) {
    //         Ok(t) => println!("{}", t),
    //         Err(s) => println!("{}", s),
    //     };
    // }
    // if matches.is_present("ll1") {
    //     match ll1::ll_table_from_grammar(&grammar) {
    //         Ok(t) => println!("{}", t),
    //         Err(s) => println!("{}", s)
    //     };
    // }

    // if matches.is_present("which") {
    //     if lr0::is_lr0_grammar(&grammar) {
    //         println!("is LR(0) grammar");
    //         // println!("{}", lr0::lr_table_from_grammar(&grammar).unwrap());
    //     } else if slr1::is_slr1_grammar(&grammar) {
    //         println!("is SLR(1) grammar");
    //         // println!("{}", slr1::lr_table_from_grammar(&grammar).unwrap());
    //     } else if lr1::is_lr1_grammar(&grammar) {
    //         if lalr1::is_lalr1_grammar(&grammar) {
    //             println!("is LALR(1) grammar");
    //             // println!("{}", lalr1::lalr_table_from_grammar(&grammar).unwrap());
    //         } else {
    //             println!("is LR(1) grammar");
    //             // println!("{}", lr1::lr_table_from_grammar(&grammar).unwrap());
    //         }
    //     }
    // }
}
