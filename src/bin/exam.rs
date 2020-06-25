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
use grammar::token::{TokenStream, Token};

use grammar::optimization::dominant;
use grammar::optimization::{ud, du};

use grammar::automaton::dfa::DFA;
use grammar::automaton::nfa::NFA;

use clap::{Arg, App};

use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::io;
use std::env;

fn input_token_stream() -> TokenStream {
    println!("enter token (split by space):");

    let mut input = String::new();
    io::stdin().read_line(&mut input);

    let mut ans: Vec<Token> = Vec::new();

    for token in input.split_whitespace() {
        let t = Token::new(String::from(token), String::from(token), 0, 0);
        ans.push(t);
    }

    ans.push(Token::new(String::from("EOF"), String::new(), 0, 0));
    TokenStream {
        stream: ans
    }
}

fn main() {
    let matches = App::new("Exam Cheater")
        .version("0.4.1")
        .author("wormtql <584130248@qq.com>")
        .arg(Arg::with_name("lr0_family")
                .long("lr0f")
                .takes_value(false)
                .help("print LR(0) item set family"))
        .arg(Arg::with_name("lr1_family")
                .long("lr1f")
                .takes_value(false)
                .help("print LR(1) item set family"))
        .arg(Arg::with_name("ffset")
                .long("ffset")
                .takes_value(false)
                .help("print first and follow set"))
        .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("input grammar file"))
        .arg(Arg::with_name("is_ll1")
                .long("is-ll1")
                .takes_value(false)
                .help("determine is LL(1) grammar"))
        .arg(Arg::with_name("is_lr0")
                .long("is-lr0")
                .takes_value(false)
                .help("determine is LR(0) grammar"))
        .arg(Arg::with_name("is_slr1")
                .long("is-slr1")
                .takes_value(false)
                .help("determine is SLR(1) grammar"))
        .arg(Arg::with_name("is_lr1")
                .long("is-lr1")
                .takes_value(false)
                .help("determine is LR(1) grammar"))
        .arg(Arg::with_name("is_lalr1")
                .long("is-lalr1")
                .takes_value(false)
                .help("determine is LALR(1) grammar"))
        .arg(Arg::with_name("lr0_table")
                .long("lr0-table")
                .takes_value(false)
                .help("print LR(0) table"))
        .arg(Arg::with_name("slr1_table")
                .long("slr1-table")
                .takes_value(false)
                .help("print SLR(1) table"))
        .arg(Arg::with_name("lr1_table")
                .long("lr1-table")
                .takes_value(false)
                .help("print LR(1) table"))
        .arg(Arg::with_name("lalr1_table")
                .long("lalr1-table")
                .takes_value(false)
                .help("print LALR(1) table"))
        .arg(Arg::with_name("ll1_table")
                .long("ll1-table")
                .takes_value(false)
                .help("print LL(1) table"))
        .arg(Arg::with_name("perform_lr0")
                .long("perform-lr0")
                .takes_value(false)
                .help("perform LR(0) analysis"))
        .arg(Arg::with_name("perform_slr1")
                .long("perform-slr1")
                .takes_value(false)
                .help("perform SLR(1) analysis"))
        .arg(Arg::with_name("perform_lr1")
                .long("perform-lr1")
                .takes_value(false)
                .help("perform LR(1) analysis"))
        .arg(Arg::with_name("perform_lalr1")
                .long("perform-lalr1")
                .takes_value(false)
                .help("perform LALR(1) analysis"))
        .arg(Arg::with_name("perform_ll1")
                .long("perform-ll1")
                .takes_value(false)
                .help("perform LL(1) analysis"))
        .arg(Arg::with_name("dominant")
                .long("dom")
                .takes_value(false)
                .help("calculate dominant set of nodes"))
        .arg(Arg::with_name("ud")
                .long("ud")
                .takes_value(true)
                .number_of_values(2)
                .help("calculate IN and OUT"))
        .arg(Arg::with_name("du")
                .long("du")
                .takes_value(true)
                .number_of_values(2)
                .help("calculate INL and OUTL (live variables)"))
        .arg(Arg::with_name("nfa2dfa")
                .long("nfa2dfa")
                .takes_value(false)
                .help("convert NFA to DFA"))
        .arg(Arg::with_name("dfa_minimize")
                .long("dfa-minimize")
                .takes_value(false)
                .help("minimize DFA"))
        .arg(Arg::with_name("fa_erlian")
                .long("fa-erlian")
                .takes_value(false)
                .help("convert NFA to DFA and minimize DFA"))
        .get_matches();
    
    // check if seed exists, if not, cannot use any functions;
    if env::var("SEED").is_err() {
        println!("you must give a seed, (eg. set SEED=10086)");
        return;
    }
    let seed: u64 = env::var("SEED").unwrap().parse().unwrap();


    if matches.is_present("nfa2dfa") {
        let nfa = NFA::from_file(matches.value_of("file").unwrap());
        let (dfa, table) = nfa.to_dfa_with_process();

        println!("step:\n{}", table);
        println!("DFA:\n{}", dfa);
        return;
    }

    if matches.is_present("dfa_minimize") {
        let dfa = DFA::from_file(matches.value_of("file").unwrap());
        let (dfa, t1, t2) = dfa.minimize_with_process();

        println!("split:\n{}", t1);
        println!("result:\n{}", t2);
        println!("DFA:\n{}", dfa);
        return;
    }

    if matches.is_present("fa_erlian") {
        let nfa = NFA::from_file(matches.value_of("file").unwrap());
        let (dfa, table) = nfa.to_dfa_with_process();

        println!("step:\n{}", table);
        println!("NFA to DFA:\n{}", dfa);

        let (dfa2, t1, t2) = dfa.minimize_with_process();

        println!("split:\n{}", t1);
        println!("result:\n{}", t2);
        println!("DFA after minimize:\n{}", dfa);
        return;
    }

    // ud
    if matches.is_present("ud") {
        let temp: Vec<&str> = matches.values_of("ud").unwrap().collect();
        
        let graph = dominant::graph_from_file(temp[0]);
        let gen_kill = ud::load_gen_kill_from_file(temp[1]);

        let (ans, table) = ud::calc_with_process(&graph, &gen_kill);
        let table2 = ud::get_table(&ans);

        println!("process:\n{}", table);
        println!("{}", table2);
        return;
    }

    // du
    if matches.is_present("du") {
        let temp: Vec<&str> = matches.values_of("du").unwrap().collect();

        let graph = dominant::graph_from_file(temp[0]);
        let use_def = du::load_use_def_from_file(temp[1]);

        let (ans, table) = du::calc_with_process(&graph, &use_def);
        let table2 = du::get_table(&ans);

        println!("process:\n{}", table);
        println!("{}", table2);
        return;
    }



    // input file
    let input_file = matches.value_of("file").unwrap();


    if matches.is_present("dominant") {
        let graph = dominant::graph_from_file(&input_file);
        let ans = dominant::dominant(&graph);
        let table = dominant::get_table(&ans);

        let lp = dominant::calc_loop(&graph, &ans);

        println!("dom:\n{}", table);
        println!("loop:\n{:?}", lp);
        return;
    }

    
    let g = Grammar::from_advanced_file(&input_file).unwrap();


    if matches.is_present("perform_lr0") {
        let token_stream = input_token_stream();
        let lr_table = lr0::lr_table_from_grammar(&g).unwrap();
        println!("LR Table:\n{}", lr_table);
        let (parse_tree, table) = lr_table.analysis_with_process(&token_stream);
        println!("steps:\n{}", table);
        if parse_tree.is_some() {
            println!("parse tree:\n{}", parse_tree.unwrap());
        }
        
        return;
    }

    // need seed
    if matches.is_present("perform_slr1") {
        let token_stream = input_token_stream();
        let lr_table = slr1::lr_table_from_grammar_with_seed(&g, seed).unwrap();
        println!("LR Table:\n{}", lr_table);
        let (parse_tree, table) = lr_table.analysis_with_process(&token_stream);
        println!("steps:\n{}", table);
        if parse_tree.is_some() {
            println!("parse tree:\n{}", parse_tree.unwrap());
        }

        return;
    }

    // need seed
    if matches.is_present("perform_lr1") {
        let token_stream = input_token_stream();
        let lr_table = lr1::lr_table_from_grammar_with_seed(&g, seed).unwrap();
        println!("LR Table:\n{}", lr_table);
        let (parse_tree, table) = lr_table.analysis_with_process(&token_stream);
        println!("steps:\n{}", table);
        if parse_tree.is_some() {
            println!("parse tree:\n{}", parse_tree.unwrap());
        }
        
        return;
    }

    // need seed
    if matches.is_present("perform_lalr1") {
        let token_stream = input_token_stream();
        let lr_table = lalr1::lalr_table_from_grammar_with_seed(&g, seed).unwrap();
        println!("LR Table:\n{}", lr_table);
        let (parse_tree, table) = lr_table.analysis_with_process(&token_stream);
        println!("steps:\n{}", table);
        if parse_tree.is_some() {
            println!("parse tree:\n{}", parse_tree.unwrap());
        }
        
        return;
    }

    if matches.is_present("perform_ll1") {
        let token_stream = input_token_stream();
        let ll_table = ll1::ll_table_from_grammar(&g).unwrap();
        println!("LL Table:\n{}", ll_table);
        let (parse_tree, table) = ll_table.analysis_with_process(&token_stream, g.origin.clone());
        println!("steps:\n{}", table);

        return;
    }

    // display LR(0) family
    // need seed
    if matches.is_present("lr0_family") {
        let ans = LR0ItemSetFamily::from_grammar_with_seed(&g, seed);
        println!("{}", ans);
    }

    // display LR(1) family
    // need seed
    if matches.is_present("lr1_family") {
        let ans = LR1ItemSetFamily::from_grammar_with_seed(&g, seed);
        println!("{}", ans);
    }

    // display FIRST and FOLLOW set
    if matches.is_present("ffset") {
        let ans = FirstFollowSet::from_grammar(&g);
        println!("{}", ans);
    }

    // determine is LL(1) grammar
    if matches.is_present("is_ll1") {
        if ll1::is_ll1_grammar(&g) {
            println!("yes");
        } else {
            println!("no");
        }
    }

    // determine is LR(0) grammar
    if matches.is_present("is_lr0") {
        if lr0::is_lr0_grammar(&g) {
            println!("yes");
        } else {
            println!("no");
        }
    }

    // determine is SLR(1) grammar
    if matches.is_present("is_slr1") {
        if slr1::is_slr1_grammar(&g) {
            println!("yes");
        } else {
            println!("no");
        }
    }

    // determine is LR(1) grammar
    if matches.is_present("is_lr1") {
        if lr1::is_lr1_grammar(&g) {
            println!("yes");
        } else {
            println!("no");
        }
    }

    // determine is LALR(1) grammar
    if matches.is_present("is_lalr1") {
        if lalr1::is_lalr1_grammar(&g) {
            println!("yes");
        } else {
            println!("no");
        }
    }

    // display LR(0) table
    // need seed
    if matches.is_present("lr0_table") {
        if lr0::is_lr0_grammar(&g) {
            let table = lr0::lr_table_from_grammar_with_seed(&g, seed).unwrap();
            println!("{}", table);
        } else {
            println!("not LR(0) grammar");
        }
    }

    // display SLR(1) table
    // need seed
    if matches.is_present("slr1_table") {
        if slr1::is_slr1_grammar(&g) {
            let table = slr1::lr_table_from_grammar_with_seed(&g, seed).unwrap();
            println!("{}", table);
        } else {
            println!("not SLR(1) grammar");
        }
    }

    // display LR(1) table
    // need seed
    if matches.is_present("lr1_table") {
        if lr1::is_lr1_grammar(&g) {
            let table = lr1::lr_table_from_grammar_with_seed(&g, seed).unwrap();
            println!("{}", table);
        } else {
            println!("not LR(1) grammar");
        }
    }

    // display LALR(1) table
    // need seed
    if matches.is_present("lalr1_table") {
        if lalr1::is_lalr1_grammar(&g) {
            let table = lalr1::lalr_table_from_grammar_with_seed(&g, seed).unwrap();
            println!("{}", table);
        } else {
            println!("not LALR(1) grammar");
        }
    }

    // display LL(1) table
    if matches.is_present("ll1_table") {
        if ll1::is_ll1_grammar(&g) {
            let table = ll1::ll_table_from_grammar(&g).unwrap();
            println!("{}", table);
        } else {
            println!("not LL(1) grammar");
        }
    }
}