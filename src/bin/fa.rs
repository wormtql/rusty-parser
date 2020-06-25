use grammar::automaton::dfa::DFA;
use grammar::automaton::nfa::NFA;

use clap::{Arg, App};

fn main() {
    let matches = App::new("Exam Cheater: Automaton")
        .version("0.1.0")
        .author("wormtql <584130248@qq.com>")
        .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .required(true)
                .help("input DFA/NFA file"))
        .arg(Arg::with_name("nfa2dfa")
                .long("nfa2dfa")
                .takes_value(false)
                .help("convert NFA to DFA"))
        .arg(Arg::with_name("dfa_minimize")
                .long("dfa-minimize")
                .takes_value(false)
                .help("minimize DFA"))
        .get_matches();


    if matches.is_present("nfa2dfa") {
        let nfa = NFA::from_file(matches.value_of("file").unwrap());
        let (dfa, table) = nfa.to_dfa_with_process();

        println!("{}", table);
        println!("DFA:\n{}", dfa);
    }

    if matches.is_present("dfa_minimize") {
        let dfa = DFA::from_file(matches.value_of("file").unwrap());
        let (dfa, t1, t2) = dfa.minimize_with_process();

        println!("split:\n{}", t1);
        println!("result:\n{}", t2);
        println!("DFA:\n{}", dfa);
    }

    // let dfa = DFA::from_file("automaton_test/test.dfa");
    //let nfa = NFA::from_file("automaton_test/test.nfa");
    //println!("{}", nfa);
    
    // let (dfa2, t1, t2) = dfa.minimize_with_process();

    // println!("{}", t1);
    // println!("{}", t2);
    // println!("{}", dfa2);

    // println!("{}", table);
}