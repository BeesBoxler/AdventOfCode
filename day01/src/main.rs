fn main() {
    // How it should be done ⬇️
    //
    // let mut input = String::new();
    // io::stdin().read_line(&mut input)
    //     .expect("Please provide input.");
    //
    // However there seems to be a limit to the input length in the console so I am hardcoding the value. (Sorry)
        let input = String::from("5162992814911695127194252761945964242912687126971558636518469379259284569588136244281562184683314238584226
                                  1347196216575642383775685651975452498575976374755971125797736122835767829357269883975444475289883531339981
                                  5748562519958329927911861654784216355489319995566297499836295985943899373615223375271231128914745273184498
                                  9152414883937616767999143852654599839237431465554651778864919799624659188883966642336932439839694126825617
                                  9962878956929437455457567736821972414253678964912175858299134553763988885811376373851851118443985422338686
                                  8764189133964543721941169786274781775658991329331759679943342217578532643519615296424396487669451453728113
                                  1147482171778268749534664354361291652953791572263457867568999357473367851617454879337215272393941187215171
                                  9584918667681423288741317558732721414487689824857124851712179676624881736661433391515479698361217428123784
                                  6165129114988453188844745119798643314857871527757831265298846833327863781341559381238458322786192379487455
                                  6715637571235342534635634217161386416119156862473434171266553173786393141684613456134272627866246894984855
                                  9994233681399572514516935594261667281279217455686643615837593898873872125366477258457738455869647754623218
                                  9312287262439452141564522329987139692281984783513691857538335537553448919819545332125483128878925492334361
                                  5621926216729938684795666885647522261117844866197895883181717459952536458868338726654472412453299356438838
                                  9244752428664229695535424947881511651731583217992549481874847816431766947165446486711192467696116216284123
                                  2473474394739793968624974397916495667233337397241933765513777241916359166994384923869741468174653353541147
                                  6166453939176945818111939773119817525545514996292198733914934268838865362194558483544264615629952841623239
                                  6177364458181563377976263474533956519679872484772278166694862623163163214437187315487257561563632296535325
                                  4642186897127423352618879431499138418872356116624818733232445649188793318829748789349813295218673497291134
                                  1643957396656672554433663832996699736895281882643863735914241497844736984873153166766371653179726489161167
                                  55224598519934598889627918883283534261513179931798591959489372165295");
    let input = input.trim();
    calculate_part_one(input.as_bytes())
}

fn calculate_part_one(bytes: &[u8]) {
    let mut sum:u64 = 0;
    for (i,v) in bytes.iter().enumerate() {
        let this = v - 48;
        let next_i = (i + 1) % bytes.len();
        let next = bytes[next_i] - 48;

        if this == next {
            sum += this as u64;
        }
    }
    println!("Part one: {}",sum)

}

fn calculate_part_two(bytes: &[u8]) {
    let mut sum:u64 = 0;
    for (i,v) in bytes.iter().enumerate() {
        let this = v - 48;
        let next_index = i + (bytes.len() / 2) % bytes.len();
        let next = bytes[next_index] - 48;

        if this == next {
            sum += this as u64;
        }
    }

    println!("Part two: {}", sum)
}