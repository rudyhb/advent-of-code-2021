use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

pub(crate) fn run() {
    let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
//     let input = _get_input();

    let data: Vec<Sentence> = input.split('\n').map(|s| s.into()).collect();

    println!("count 1,4,7,8: {}", data.iter().map(|s| s.output.iter().filter(|s| s.is_1_4_7_8()).count()).sum::<usize>());

    let data: Vec<InputOutput> = input.split('\n').map(|s| s.parse().unwrap()).collect();

    let mut sum = 0u32;
    for sentence in data {
        let mut mapping_candidates: MappingCandidates = Default::default();
        for word in sentence.input.iter().chain(sentence.output.iter()) {
            mapping_candidates.reduce(word);
        }
        println!("mapping candidates:\n{:?}", mapping_candidates);
        let mapping: Mapping = mapping_candidates.try_resolve().expect("could not resolve candidates");

        println!("mapping: {:?}", mapping);
        let output = mapping.get_value(sentence.output);
        println!("output: {}", output);
        sum += output;
    }
    println!("sum: {}", sum);
}

struct Sentence<'a> {
    _input: Vec<Word<'a>>,
    output: Vec<Word<'a>>,
}

impl<'a> From<&'a str> for Sentence<'a> {
    fn from(s: &'a str) -> Self {
        let map_words = |s: &'a str| s.trim().split_whitespace().map(|w| Word(w)).collect::<Vec<Word>>();
        let mut parts = s.split('|');
        Self {
            _input: map_words(parts.next().unwrap()),
            output: map_words(parts.next().unwrap()),
        }
    }
}

struct Word<'a>(&'a str);

impl<'a> Word<'a> {
    pub fn is_1_4_7_8(&self) -> bool {
        match self.0.len() {
            2 | 4 | 3 | 7 => true,
            _ => false
        }
    }
}

#[derive(Copy, Clone)]
struct Wiring(u8);

impl Wiring {
    pub fn _is_1_4_7_8(&self) -> bool {
        match count_set_bits(self.0) {
            2 | 4 | 3 | 7 => true,
            _ => false
        }
    }
    pub fn from(bits: [u8; 7]) -> Self {
        Self(bits.iter().enumerate().map(|(i, &bit)| bit * 2u8.pow(i as u32)).sum())
    }
    pub fn to_bits(&self) -> [u8; 7] {
        let mut result: [u8; 7] = Default::default();
        for i in 0usize..7 {
            if self.0 & 2u8.pow(i as u32) != 0 {
                result[i] = 1;
            }
        }
        result
    }
    pub fn position_of_set_bits(&self) -> impl Iterator<Item=usize> {
        self.to_bits().into_iter().enumerate().filter(|(_, b)| *b == 1).map(|(i, _)| i)
    }
    pub fn map_wire(&self, mapping: &Mapping) -> Self {
        Self((0usize..7)
            .filter(|&i| self.0 & 2u8.pow(i as u32) != 0)
            .map(|i| 2u8.pow(mapping.get(i) as u32))
            .sum::<u8>())
    }
}

impl From<Wiring> for u32 {
    fn from(num: Wiring) -> Self {
        match num.to_bits() {
            [1, 1, 1, 0, 1, 1, 1] => 0,
            [0, 0, 1, 0, 0, 1, 0] => 1,
            [1, 0, 1, 1, 1, 0, 1] => 2,
            [1, 0, 1, 1, 0, 1, 1] => 3,
            [0, 1, 1, 1, 0, 1, 0] => 4,
            [1, 1, 0, 1, 0, 1, 1] => 5,
            [1, 1, 0, 1, 1, 1, 1] => 6,
            [1, 0, 1, 0, 0, 1, 0] => 7,
            [1, 1, 1, 1, 1, 1, 1] => 8,
            [1, 1, 1, 1, 0, 1, 1] => 9,
            _ => panic!("out of range wiring to u8")
        }
    }
}

impl Debug for Wiring {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.position_of_set_bits().map(|i| (('a' as usize) + i) as u8 as char).collect::<String>())
    }
}

impl From<u32> for Wiring {
    fn from(num: u32) -> Self {
        let bits = match num {
            0 => [1, 1, 1, 0, 1, 1, 1],
            1 => [0, 0, 1, 0, 0, 1, 0],
            2 => [1, 0, 1, 1, 1, 0, 1],
            3 => [1, 0, 1, 1, 0, 1, 1],
            4 => [0, 1, 1, 1, 0, 1, 0],
            5 => [1, 1, 0, 1, 0, 1, 1],
            6 => [1, 1, 0, 1, 1, 1, 1],
            7 => [1, 0, 1, 0, 0, 1, 0],
            8 => [1, 1, 1, 1, 1, 1, 1],
            9 => [1, 1, 1, 1, 0, 1, 1],
            _ => panic!("out of range u8 to wiring")
        };
        Wiring::from(bits)
    }
}

impl FromStr for Wiring {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Wiring(s.chars().into_iter().map(|c| 2u8.pow(c as u32 - 'a' as u32)).sum::<u8>()))
    }
}

struct InputOutput {
    input: Vec<Wiring>,
    output: Vec<Wiring>,
}

impl FromStr for InputOutput {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map_words = |s: &str| s.trim().split_whitespace().map(|w| w.parse().unwrap()).collect::<Vec<Wiring>>();
        let mut parts = s.split('|');
        Ok(Self {
            input: map_words(parts.next().unwrap()),
            output: map_words(parts.next().unwrap()),
        })
    }
}

struct MappingCandidates([u8; 7]);

impl Debug for MappingCandidates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let map_char = |c: u8| (c + 'a' as u8) as char;
        write!(f, "{{\n{}\n}}", self.0.iter().enumerate().flat_map(|(i, &c)| format!("{} -> [{}]\n", map_char(i as u8), Wiring(c).position_of_set_bits().map(|c| map_char(c as u8)).collect::<String>()).chars().collect::<Vec<char>>()).collect::<String>().trim())
    }
}

impl Default for MappingCandidates {
    fn default() -> Self {
        Self([127u8; 7])
    }
}

#[derive(Debug)]
struct Mapping([u8; 7]);

impl From<&MappingCandidates> for Mapping {
    fn from(c: &MappingCandidates) -> Self {
        let mut result = [0u8; 7];
        for i in 0..result.len() {
            result[i] = get_byte_index(c.0[i]);
        }
        Self(result)
    }
}

impl Mapping {
    pub fn get(&self, i: usize) -> u8 {
        self.0[i]
    }
    pub fn get_value(&self, words: Vec<Wiring>) -> u32 {
        words.iter().rev().enumerate()
            .map(|(i, word)| {
                let val: u32 = word.map_wire(self).into();
                10u32.pow(i as u32) * val
            })
            .sum()
    }
}

fn get_byte_index(b: u8) -> u8 {
    for i in 0..7 {
        if 1 << i & b != 0 {
            return i;
        }
    }
    panic!("get_byte_index out of bounds");
}

impl MappingCandidates {
    pub fn reduce(&mut self, wiring: &Wiring) {
        let targets: Vec<Wiring> = match count_set_bits(wiring.0) {
            2 => vec![1u32],
            3 => vec![7u32],
            4 => vec![4u32],
            5 => vec![2u32, 3, 5],
            6 => vec![0u32, 6, 9],
            7 => vec![8u32],
            _ => panic!("target out of range")
        }.into_iter().map(|wiring| wiring.into()).collect();
        let empty_target_bits: Wiring = Wiring::from(match count_set_bits(wiring.0) {
            2 => [1u8, 1, 0, 1, 1, 0, 1],
            3 => [0u8, 1, 0, 1, 1, 0, 1],
            4 => [1u8, 0, 0, 0, 1, 0, 1],
            5 => [0u8, 1, 1, 0, 1, 1, 0],
            6 => [0u8, 0, 1, 1, 1, 0, 0],
            7 => [0u8; 7],
            _ => panic!("target out of range")
        });

        self.reduce_to_targets(wiring, &targets, &empty_target_bits);
    }
    pub fn try_resolve(&mut self) -> Option<Mapping> {
        loop {
            if Self::is_complete(self) {
                break;
            }
            let already_resolved: Vec<(usize, u8)> = self.0.iter().enumerate().map(|(i, &b)| (i, b)).filter(|(_, b)| count_set_bits(*b) == 1)
                .collect();
            let any_to_update = already_resolved.iter().any(|&(i, b)| self.0.iter().enumerate().any(|(j, &b0)| i != j && b & b0 != 0));
            if !any_to_update {
                return None;
            }
            for (reference_i, to_remove_b) in already_resolved.into_iter() {
                for (_, b) in self.0.iter_mut().enumerate().filter(|(i, _)| *i != reference_i) {
                    *b -= *b & to_remove_b;
                }
            }
        }
        Some((&*self).into())
    }
    pub fn is_complete(&self) -> bool {
        self.0.iter().all(|&b| count_set_bits(b) == 1)
    }
    fn _reduce_to_target(&mut self, source: &Wiring, target: &Wiring) {
        for source_i in source.position_of_set_bits() {
            self.0[source_i] &= target.0;
        }
    }
    pub fn mapped_space(&self, source: &Wiring) -> u8 {
        source.position_of_set_bits().
            fold(0u8, |aggr, current| self.0[current] | aggr)
    }
    fn reduce_to_targets(&mut self, source: &Wiring, targets: &Vec<Wiring>, empty_target_bits: &Wiring) {
        let target_space = self.mapped_space(source);
        let reduced_targets: Vec<&Wiring> = targets.iter().filter(|target| target.0 & target_space == target.0).collect();
        println!("source: {:?}, targets: {:?}, reduced targets: {:?}", source, targets, reduced_targets);
        let aggr_target = reduced_targets.iter().fold(0u8, |val, current| val | current.0);
        let source_bits: HashSet<usize> = source.position_of_set_bits().collect();
        let fully_captures = Wiring(aggr_target).position_of_set_bits().count() == source_bits.len();
        for i in 0..7 {
            if source_bits.contains(&i) {
                self.0[i] &= aggr_target;
            } else {
                self.0[i] &= empty_target_bits.0;
                if fully_captures {
                    self.0[i] &= !aggr_target;
                }
            }
        }
    }
}

fn count_set_bits(number: u8) -> u8 {
    let mut count = 0u8;
    let mut n = number;
    while n != 0 {
        n = n & (n - 1);
        count += 1;
    }
    count
}

fn _get_input() -> &'static str {
    "dbcfeag cgaed fe bfgad aefcdb efa efgda gcef dcaebg dfeagc | fae cfge fae baefdc
ea bfecadg bgafcd deac ebcag eab debfag eabgdc bcgfe bagdc | cdbagf fagbcd bae fcegb
fbgdec cbgeaf cbfag bd bgda cgbfda dbf fecda bfadc cbedagf | gecfdb cbfga bacgef dfb
geabdf cfbge dcbeaf aebdf dgea acdgfbe fag gfbea ag cabdfg | dfeabc dbaefc fdaebc fga
gcae cefdg abdcgfe cg fcg afcegd dbgfea agdfe dcafbg bcdef | gc caefdg gcf egdcf
efgcb fbac dbefg bce efacg fbgcdae cb egfbca dgfaec dcgbea | fgecb bcfa bc efcag
efcgdb da bdefg dfegba fbacg gbcdea dbafceg adb feda fadgb | da ad cgabde cebdga
ca fcabegd facd agbed acg cfadeg acdeg agcefb cdfgeb efcdg | cdeafg cag efgadc gacbfe
fdacgb cfa cfage gabce dfagce edagfb dcbfeag fc fced gfdae | gabdef fdegba gaedcf fac
acgdbf faedcbg fa cbgdae fgbdae aedbg adf fbea cfdge fedga | fad egdcf feab bfcagd
ged cadgf gcdafeb cabgfd cgedf aedc ebfcg faebgd ed fcadeg | dfgca gabefd dafgc ecfgd
bcedga dceba gcadfbe dfec adcbfe fda fabge df fbdae bcfgda | adgcfbe fbgae dgebafc gacedb
fgcabde edbcfg efcdga dfbcea dbaeg fea fa dbaef acfb dbfec | aecbfd aedcfb bdaef abfc
cfbd gefad dce fdeac dc ecabfd dgabce gabdecf afbce fagceb | bdfc bcgaed bgdace afegd
acfgde egdfb dcaf gecfd ecdbgfa gcdbae cefgba dcg gcefa cd | eadcbg cdfega fdgeac dgc
cadfgb edcgbfa dc cgd fegcb dfabg cdaebg dafc fcbdg gbdeaf | afdc adcf befdag gfcdb
deafg ebafdcg gcdfba bge eb fgcbd gbefac bdce fdebg gdcfeb | cbdagf bdce gdfeb fgead
acbdef bg agebdf afbcg fgbeca edafcgb eacfb gba ebgc fgcad | begdaf gcbe ebdfag dabefg
ga agc bdegcaf gefbc cfdea dafgec fagce dgebca aefbdc gdfa | acfedbg bfaecdg gca facedg
abdg ad egcabf ebagf afd gdfceab dcebf dagbef cdgafe deabf | bfedc cbedf gdab afd
efbadc cafbd eba fegda fcbe dbefa efcbdag adbcge badfcg be | be agdfe cedbag dcbeaf
adegb cgafb badgcfe fged fgaebd eabgf bceafd eaf eacbdg fe | bdgeac adgbfe bdgae acfgbed
dbfega befag gbfdec gacfed fcgdabe bgf deagf bf dafb bceag | bdfa cagbe gceba agbdfe
fecagd cbfeagd fdecb fg fgbde ebgda cagdbe fge gfab agefdb | geabcd gdbea ebgafd egdab
ab agdcfe dcfgb cba adefcb adfbc bdae ebacgf aegbdcf fcade | gfecda dfabc edab bfcad
bcfae ebd gefdacb fdceg dbaegc gdbf gcefbd cedfb adfgce bd | gebdcf bd dgfb ebdcgf
gbcafde cdaef dfe ecbf egcda fadgcb bfdac dacfbe bgefda ef | dcgafb dfaebg fbec agbdfc
fe cfedab dcfgeb gdbef gadeb cegf fgcbd fbe abdcgf ebacgfd | debfg abecfd adbefc gdcbfe
ad cbfeda acegfb gdcaeb adcgfbe bda abefc dcabf fgcdb afde | gdcbf cedbaf dbagec becfa
acegb efcgab edcfgab aebdc gce ge fage egcdfb agfbc cbdfga | eg cafegb fcbeag acbgef
bcaf agefdc bcegaf eaf bdgfe agfdecb efgab af abegc dgacbe | defbg dgaebc eaf cebga
edabcf bdeg gd cadebg dgcab gfcaed febdgca cdg beacd cagbf | befadcg fbgca afdecg bgceda
gadfeb ag gfda cbeda fbeacg bgdcaef dgfeb eag gbdea fegcbd | cabefg degbcf gdbef bcdgef
efbcda cgfd dgefac dfcgbae degaf adg debacg aedcf gd aebgf | ecfad cgbade aegdcb gfbae
bdc cdfgb cabgf gabd fdebagc eabgcf bd abdcef gfcabd fgedc | faecdb fgecd cbfdag bcfadg
dfagbec ceg egfad fdeabg acedg afedcg gc bcgaef eacdb fdcg | gcfd eadbc dbegcfa eabcd
gfb egfba eadcfb gf adfg dgebfa dacfegb eagcb cbedgf fadeb | edfba afgebd adbegf efcbda
fec caegd efgb cgaedfb gfbdec gfdbc ef cfdgab fdcge fbaedc | abcdef gcfbd bfegdac gcead
gefdba abcef ead gedcab ad bfdecg cbged ebcad cafbdeg cadg | defcgb ade bgecd ceabf
cabg adgefbc cb ebafg febgc fbc gdcfe cafedb egfabc aebdgf | bcdafeg ecbafd adbecf fbcge
acged febgd agdcfb bdceg gdbecf bc bgc cfeb cfgbaed bgefad | dceag efbc cbfdeg ebcdfg
gab adgcbef cebfg abdce dcbefg bafdgc abefgc aegf ag cgbea | gaceb aegf fgcbe cgebf
fa baf cadfbge dbgef ebacgd dfceba adbec fdeba cfae dacgfb | dabcef dafcbe acgdbe ecdfab
fgeca gecadfb aecdfb fb gcbed dfgb cgdbea ecfgb gfbdec efb | abcedg bf bf agedcfb
fg aefgd bafcde gcfe bdgae gfa fedca cagdfb cfaedg ebgacfd | dcfgab dgfea gf cafdgb
afbcge debgc cafdbg ae eadf afdgb gbdeaf aeb cgfebad abgde | deaf cbafge cbegd aedbg
fbc afcegb fadgc fbeda cbed ecdafb cfdab cb fegdcab bdaefg | cgfad ecbd edafb cgafd
fegbcd abcf gfabedc bgcad gfacd gedab efdagc dfgcab cdb bc | fbca cdbag cdfgab cgedbfa
gfca gbdef eadcfg ebdcag dcg afecd gc decafbg fcegd bcfade | gcefd bcdafge cdefg edfgc
gdae ad gecfd dcfaeb dfegcb gdcaf fdacge adf agbcf cebfgad | dgebafc edfgc da cbgafed
ceg febgdca bfgdea gc adcfge cefbag agdc edfcb dfgce gadef | ceg ebdagf agdfe afbgce
dabegcf bcdfag gdecb gdbeca gaebd fbdce bdagef cg bgc ecag | dfcabg abfdge gadbe cg
dacefb efc gfaebc gbfe egcda agfbdc beadfcg fe fbcga cgafe | ef fbaedc ef efc
cbgae faedbg fegab agc ebdgc eacf bgeafc ca cgdbafe gadcbf | eagfb aebgf gcbfad gfeba
dcf dacbg bdcaef dfeacg gacfe cdfag fgde faecgb daebcgf df | fcd fecadb egacf deabcf
egdfa dfaec gdab ga agf dbefga dgebf afbgecd bgafec bgefcd | fegbd gdfbe begdfc efcbadg
cfd edafbc afbd cadbeg fgecbad gcaef dacef df dbcae efcdgb | adbf cabde baegdc cgbeda
cbag gc cdgbe defbg agebdfc becda dbgcae cdg efacbd fecgad | fcegabd abgc dacbe cdaebg
bf dacgf dfb befc fbedcg gebdc dbacefg abdegf gcdfb bedacg | bfec feagcbd egdcb daegbf
aebgfdc gadcf gcdfeb dagce agdeb acfe bgacdf dec gfecad ec | ceadg edcag fdcga cdbagef
befdgca dgcbf fedagc be bfe dcefb ecfdba acfed ebca ebgfda | dfbcae bafgdce bgcdf egbdfa
bfa bdagfc dbega bfgea eagfdb dbgace bcefg dcbgeaf fa defa | ebcgdaf aecgdbf fgcadb fa
agcdeb cgf agbdcef bgdfe efac cgefb abefgc cagdbf fc gebca | fegbc acfe becga deagbc
adbfce agebd eadcbfg ega abedf fgab dgebc dgfcea ag agdebf | gdfcabe ebadf ga bfag
gcebfd feacdbg acegfb afgce gbac geb cdaefg daefb gb fgbae | agefb gb adfbe efabd
gadbfc ebgad dgecba fgae eacgdbf af ebcfd afd bdeaf gadefb | fa bdgaef gdbacf af
bfeacd edgcabf dg gcabd cdaeb acfgb bcgdae dgae cgd bgfdec | bedac acbed abcdfge dgc
cdage dgabec cbadg ge acfde edg becfagd gbec adbcgf fbeadg | cdbgea cgdaeb cegad gdeac
ab gbaf cdegba fceadbg gfbecd bca eadcf fgeabc becfa cebfg | cagfeb cefabg fcbeg eadcgb
afdec bcaefg ca abcd efcdg aec aebdf cdafbe begdaf eadgbfc | cdfeg afdce edfac gdfabe
faebd fed df bgdcaef cbdega gdbeaf fbecgd badge afebc fadg | dcebfg dgaf fd cdegfba
abdgf efg fe gdeabf cdbge cfadeg gdbef cafgdeb fbea bgcadf | bgdfe fabe gbfadc gbfcda
agdfe cfbd dgfca cd abgfc cdgbea gbecfa gbadcef cda dfgbac | cegfba fbcd bfdc dca
acdb bdcega dbfcge aedfgb dec fcega dc adegc fgceabd gaedb | badeg bdecgf cd cdageb
abe ab dbecfag adgec agbf eadbg cbeafd fbcegd gadbfe bfgde | ebgfd efdgb gceadbf ba
dgfceba egbcaf fgc bfgdca cbgfe cg faebc agce gdfeb debcaf | bedfac edbfg aegc eabcdgf
bdfac dcfeb dbcefa dbgcaef fbae egdcb fgaecd fec cdbafg ef | bdfagc dcfabg gdbfeca eabf
gfca dbcaf bfcgd fcgdeab degcb abcedf cbagfd gfb fg fdgabe | gcaf bfg fbg gf
efcagd dbag fgd defgacb aebfg dbgfe bdeafg edfbc gd fgecab | fbeag ecgfabd degacf debgfac
efcbg bacfe gbdce fge acegbdf cgdf aebdfg dgbcae ebcdgf gf | caefb cbfdage ecfgb bgedc
fgdeb dcbfg fgae fbeagd edbfa aegdbc egb gafecdb bfdeca eg | dgbcea bdaef gfbde dfgbea
bdcaf gdafc cbf bdefa cb fgceda bdfagc fcbgde afcgedb cbga | gbca fbdca cegfdb fcagd
cfebgd cedgfa dcfabge dbfgc dgeb ge fcegb gfe abfdcg fecab | fge fabec gfbadec bfceg
aeb ab gebfd abcdef edcag cdegaf gaedbc bgfecda debag gbac | beacgd bdfeg afgecd gedbf
ab gebfadc acfge edafbc abegc dcegab gbecfd gbda cab ebcdg | dagb cegfa bca bac
gaedb ebcd fgcea cba dcgafb febadg egcab adcbge efgadcb cb | dfcagb cbed bagce dbce
ca dgfceb bfeacd fcbgd bdgca cfag gaedb abc fcbagd fgecadb | cgdfab cfag bac cbfegad
gade gafcb bfgecd bge ecbda gdcbea eg beacg fdaecb agfbecd | dgae bgfca abegc cfdebg
eac cdbef ac gfdae dagc afcedgb dcfae dafcge dbfega caebfg | gafceb dbeacgf cdga fbcde
dbagfc bgedca df afgec daf beagdfc aefcbd bedac acfed dbfe | bedca cdaef baecfd befcad
fdacb baedc acedfb bacedg fd gfcab ecdf daf fedgba befgdac | egabcd cedab eabdfg adf
efdgc dfecga adfbe ac cage adc gfbdec cagbfd efdabgc caefd | efdab efdgcab ca cedfg
ca acgf cadbgfe egfcab gefab cea defbc fecab edbcga fbgdae | cfeba efdcbag dacebg agefcdb
bcedfa agecd ebgf fcbdg fde cgfed acgbfd cagfdeb ef cgbdef | fgeb bcfdg def ef
bfcdea ga fbdegac gecdf cfdab bcgadf facdg aegbcd gac afgb | cbadfg facdb afbg dfecba
fdcae afeg fa bceda dfebcga afcgdb dgecf acf fgceda cbfdeg | fa bdfcga fcbegd efacd
efgbcd cf bdgace fgbad dfc fceabd dbaegfc bdcgf becdg gfce | bcedaf adbegc badgec bgcfd
efcba ebdfga fda gdefbc da facde dacg fgedc gfeabcd acdefg | da bgafde cegfd edafgb
daegb cbge bgcaed afgced ge cdabe fdbcaeg fcbeda bgfda eag | gae egcb egbc egdafcb
cefda fadg bfedac bedfacg dgecfa eag ga fegcb becagd gcefa | dcfea eag cedfa bgfcaed
dcfe gbeafd df bdcaf ebfcag fcedagb cdbag dcfaeb eabcf fbd | dfce geacbf cefd ebdacf
efgbc agbd gd eadgcf becdfa bdace dge debgc adecgb bfdgcae | bgcfe bedcg abgd gdba
cdbf edbfgc feadg cd ced gaecdb degfc efcgb fcbgaed befcga | cfabgde fgdcbe efacgb abgcef
ae caegfd eadgc fedgba fecgd gbacd efac aed afcbged cgdbef | ae eacf abdegcf eda
fc afcge dbgecf adcgfe eafbg gdebacf eagcd gfc aecdgb afcd | cfg dgeac afbge edgcba
aedcg gfd cafgebd fg eadbgc adfcb cefg gabefd dacfg efcgda | dgeca afgedb cfdga aefgcd
bfgaed facedb fbec dbgca cagfbde egcfda efdac acbfd fbd fb | efbc agcdb fagdbe bacdf
fac dcae fgbda fagbce bdfac cbdfe ca cadbfe aegcdfb cebdfg | cdea cfbdeg ac afdcb
geabcfd gfcaeb gcbead eba facbg fdgcab ea bgafe fdgbe caef | bagef cafgb afbeg cbgaf
gcfdeb ceg fabcge edbag fcabdg bcgaf ceaf ec agefbdc cbega | bgafc baecgdf bcfged gbdfeac
cf adgef cfgbaed bfdc gfc egcdfb dcgaeb degbc fcedg ecagfb | fdbc cgdbe gdabce gfdce
bc afgcb cfdga dfcb cbdgea bgc efcgdab cfedga beafg bagcfd | bc bc cbfadg adcbge
egdfb fa agfbe eacf befagc fba fbedacg cadebg agcbe cfgbda | bagfec bcdgaf cbfgad dgcbfa
acgbde acgdbef cba ca bcgfd cade egdafb aebgfc agbed adgcb | fdgbc gbadef gfcdbae edfgab
egbac efacgd gfbd fbcdag fecbad dgc badcf gfcebda dbcag dg | bacdg facbegd cgd dfcgae
adgcf fbgcda efcgda dcbea fedg cfebag fae ef acdef gecbfda | fcagd gcadf gcefdba gefabc
fegdba ba cedgaf gbade bfad egfda gba cadbgef gbced fcaegb | gedaf bdega abgde bafd
gbacef gdab defgc bagfdc dfb afgcb efdbac bd dfbcg befcgad | cdgef bd gbfac ecbgfad
fgacd ef dagfce ecdfa fae adfbgc cegf febdga ceabd bgacedf | abcgfed gfce fae efacgdb
fbgdc afedg gcbafd gfedbc cfdag acd fbdeacg acbfde cgab ac | gdcbf cedfbg dacgf dcafg
becf feadb abfedc dfcageb gcbda bafedg aec daecfg acdbe ec | abdef fdeacg ace ecbf
dc cbfegda bfeadg dbcg gabfd adc dbcfga bface fgcade fdabc | dacbgf dc cdfba dc
bc bfgedc fbc adfbceg cegb degcf eabdf defcb cdfgae fcagbd | bcefd gceb fcagde cbfgad
ge cabgd egca egd adgbecf bfdagc decbag agbde deafb ebdgfc | cagdfb bgecda cgedba fabdcg
cg cfagbd gcedfa efdbc gcd gbefda fegad efdcg caegdbf agce | fgced dgfacb befdagc cefgda
bfedgc bec ce fcaebd efabg gdec cgfdb aebdfcg bfgcda cgbfe | agebf bgdcfe ec agcbdf
db dfcbge edgfca gabd dcb agfcd bafcd afbgcd ebafc edgfbca | acfdg cbdfa afceb gbda
cadegbf egdfbc adc acbdg dgbcf adgfbc da cgfdae fbda ebcga | dgacb bagdc fgcbd aegbc
geacf abc gfedba cbadfg dbcg gfdba cagfb cb bfdcae adfecbg | cb eadfgb bcfag dgcb
abdfcge adbeg badcef egcf fg fgb fbdec gebfd abgfcd gfebdc | cbeadgf cfdbag gfdbe gf
facdbe acgfbd dbc bd abgcd adceg bcegfa fdgb bfgca badcfeg | ceafdb bfcag dcgba bgdf
gcda cadbe abcedf cabeg edbgcf begcda begaf cg acbdefg gbc | dbace gdbeac dacfgbe abdce
cgf gf fadegbc afgcde abgced fecba ceagf dcagbf fgde cdaeg | gdebacf gf gdfe cfega
abdef bc gecaf dbcgfae cbfg efcba bac acebfg degcba dcegfa | bcfadeg afceb edfgac gcabed
bfcged bgdac becad fagc aegbdfc gacfdb dbgfc dafgbe ga bga | fgdecab abdcg cefabdg egdbfa
abcfg baecf agec fcgaeb agedbf ag dacfbeg bag eafcdb fcgdb | bcdfea efdabg ga fdbgc
ba baf dfbgce cgab cefgdab efcad efacbg cgfeb ebafgd cafeb | fdcegb abf baf ecabf
cegafd egdcba gbedf cgfde cfad ecf fcegadb eagdc bgaefc cf | bfged defgb gbfed efc
gedfa abdce fadbe ebfc fba badgce bf gabefcd fdbgac befcad | bcdgaf fdbgca bfa fgbdca
bacef cg dgabec agfc edfgb cge agfdbec fcabeg cgebf abfdce | decfbga cagf adcgbe dgefb
eafgd ce fbgaced ecfga eafdgc gdce cea cefdba cbfag bdgafe | eadfbg dgec efgca degfa
decgabf fecd acd acbegf agdec gabed gacbfd faceg cd fgeadc | dc dcafge abecgf dc
egd fdaebg gcbeadf fcdge ge adecf gbce dgfbac dcfgeb gdbcf | fdcae gfdbea bdfcg eg
ad cbafe dabe dfa gbfcea ceafd dcbagf fabdce bfceadg dfgec | cebaf aecbgdf cgdbfa fbaec
fc cdgbae gfcbad badcg fcdba feadb fac cgefad fdgbeac gfbc | cf cfabd caf gdfcea
fdgace edac cdfag de dcfge facebgd cgebf dfebag def bafgdc | fdgac gacfbed edgacf dgcfa
gedfac acedb fbcadge acgefb dbgf fdaeb dgafe fgaedb bf fba | fb bf bf agefcd
gefbdca fagbed cgaeb gf ebdfa gdbf afebg feg feadcb geacfd | egf fedab gaefb eagbf
efadgbc bfag edgcb cga cefab afbegc ga gaebc eafcdb daefgc | ga acg gcfbade dgecb
efcgbd gebcf gcadbf agfce fcdeagb gfa ageb aefdc ga gacbef | gcebaf geba acfde fga
ga adbg fdeac gfa fdgbcae dgfabc cfgdb fegbdc fcabge agcdf | adbcfg efadc agcdf cgdbf
febdc dacfeb gbfce cfgae bge bgdf dgcfbea baecdg gb ecfgbd | gb fcedb becdf gb
bgacfd cdfabeg adgce bcegd acdefg gb cgb bgae ecbadg bdfec | adcbfg gdfcea deafbcg ceadfg
bc gdfeac fbaec egfcba dacbeg cab gacbfde eabfd faceg fgcb | afbegc cdgabe bagecf ebcdga
fd dfe fdgae abegfd dbcaeg dfgb caedbf efcag dageb dbfeagc | dgfb gaedbf gfbd dgfcabe
agefbc aefdb abdge cdef bfdcga dacbf fe gbdcefa eabcdf efb | cdbefa adfcbg fecagdb ebf
gbade adecfg abegcdf dcbfa cgbafd cfeb dfaebc fe feabd efa | dbcfea bcfe fecb bafde
dfba dfc efbac cgead ecfgab gdfecb edacbf df afced bgecfad | afdce bcfegd efgbac adgce
abfcg egacf bceafg dabgc fgb fb fbecadg fbce gafcde gfdbea | abdgc defagb cbef egcaf
caegb decgbfa cebf edacg cgfbae gbc gfbea baedgf bc bfcdag | agebc bgcae cbg gefba
ad gadf feacg aed agbfce egcda fedcba abcdgef bcdge gecafd | gedfca ad febgac dcaebf
cegbd bgcfda dbfeag acef gcead cagfde dcfga gae fagbced ae | defgab dacgf cebdg gecdfa
fag edfcg cdfa fa ceagfd fdgabe gebac egbfcd egdcfab gcaef | acebg fag fgdec dfegcb
abe bcdfa acdbgf befd eadbc acbfgde eb fgcaeb gacde fcdbea | eadbc afdgbc bgaefc cabfdeg
afdgce dgabe dgfbe bdegfc dfg agcbdfe fd abegfc dcfb bfgce | debag bedgf ebdcfg cgebafd
dgfcea eagcbd gfceb adgb cgdeb dg eacdbf bedcafg dge abedc | ecbgf dg dbfcgea ebcad
dgecf badfce egcdfab efabd edfca ac badc gbcfae eagdfb caf | feadc febad dacef caf
gfb bacgde acbdgf afbd bdgca bf bgfdcea ecfag dgfecb fbcag | fbgdcae gdfbca fbg fgaec
bcdg dc gedabc dec cdeag cbedaf gfaebc dafge bcegdaf agceb | dfage bcefad cfbage fdgae
gc cag agbec dacbe dcbg edcbgaf gdcfae dacebg eabfg febacd | gc cbdg gc fedcba
gfdbace edbc ecfgab efcdbg ce fdcabg fedga egc cbdgf gedfc | ecdb edgfbc eagcfb cdafbg
afceg bgdeaf daceg degacb fe dfec adfgec acdgefb gcbfa gfe | efg gcfea dcfe fcgea
bdca gcfdbea dc acbedf gbfde cagefd cfebag dcf bdcfe ebafc | eafbc bcfage defcb agcbfe
cabefg adgeb fd dacf afgcb bgefdca decbfg fbd dafgb dfagbc | gbfca cbeagf cfbgad fbd
bacgef defcga afdgebc facebd afebc badc bdfeg dc dbfec dce | adbc eafdcb dabc fgedb
cdf decb fagec dc dcgbaf bacfgde gdfeb efbadg efgbdc cfged | ecgfa efgbd fedgab dc
faebgd egdfcb gecadb gdbce ag aeg cgeab gacd faecb debcafg | ag fdeabg decbg ag
eagc bdegf ga gdaef adg cbgdaf cfaegd dabfce adcegbf dfeac | acedfg agd cega bgefd
cfgbe ceadf ab cbefag cegfbd bega egdfcab fba gbfacd efcba | ceafbg abf ecfba ba
fgdce ecgbaf gebd bfg fdbac dfacge fbegdc cgbdf dacbfeg gb | aebcgfd cdabf ebdg egdcaf
febda edagf egf gdcf bcgafed fg agcbfe caged gcadeb dgaefc | cegbad gcdf gaedc ebdgca
eabgf fbdcg dfecag acebgf bafdge fbgce ecf eacb begfacd ec | fbgea gcefb fegcad decfag
fabgcd febdac acgdf beagc dgaec de eafcdg dcfbeag dae fdge | fcdgea ceadg fegd bdcaef
bcafeg ecbad dcfe ecbgafd gdefab dfabce de cadgb ebd fcaeb | bed bgeafd fgeacb ecdfba
afcgeb gcebf cdbfag cfea bdgfe cbgaf ec gfabecd bedgca egc | gce dacfbg cebfg afec
ecdafb cfeag bdgaf febga egabfc eb gbec fcebdga eab dfegac | bea gbafd eab befdac
fgbacd fbcedga fgc fg gdaf bcdgef cdfba fabced cgbfa gcabe | egcadfb adgf gbcefd gcf
dfbcga caefbd ac gaedbfc afgc afgdb bdgce dgcab fdgaeb dca | afbgd cagf adc bfedga
bdgce dbf cgeadfb fd gfbde dfbecg gaebf dacgfb cdebga dcef | fegbd fgbde bgfde fcegbd
gbda egadbf bfegd acgfde bfgaec gabef cfbde aefdbcg dg ged | abgefd gdefba deg gfeba
afebcd degcfb dce dcfgb degf bdcgafe bdcge gcabdf de cbaeg | de bagec gcdeb ebagc
eadcfg dfc gcbfade cd edafb fbcge afdbec agdbef ecfbd bdac | efdab agefdc befad defabc
agbedf degbfc begcd ceagdf cgabe ecd dcbf ebfdg abecgfd cd | fdebg fdgbe cdgeb bagdef
aefdb degf fdbaeg egb agebd bcfage gcedfab bdagc ge febacd | bgfdae fdge acfdeb deafb
cegfba dfcea gbea bac ebafc gdcebf gebcf ba gfdbac gacfdbe | febac aefcb cbfgda cafed
ec ecfab feadbg ecb dabfc cbfegd dbafgec feabg caefgb ecag | acbefg afbeg fecgabd bcfad
fd decag fagbe beafdg egdacbf gbfcde dbfa agfbec dgf geadf | bfda befgcda fgd adfb
af eabfdg edabc dgfabc efga fdbae fcagbed abf gbcdfe efgdb | gefa gfabcd fba gcdfab
dbfacg degcb gecdaf adcbg bafegcd dfbag gbdefa ac cabf agc | bcadg afbc bdgfa abcdg
efab cbedg aedfbc bfcad geacfd ea fabdcg adbfgce ade ecabd | fdceba bfceda ecgdfa aefcdg"
}

