use std::path::Path;

use crate::{ast::Compiler, lexer::Token, parser::AST};

fn test_file(file_name: &str, expected_asm: &str) {
    let version = env!("CARGO_MANIFEST_DIR");
    let path = Path::new(version).join("data").join(file_name);

    let code = std::fs::read_to_string(path).unwrap();

    let tokens = Token::parse(&code).unwrap();
    let ast = AST::parse(tokens).unwrap();
    let (circuit, _compiler) = Compiler::analyze_and_compile(ast, &code, false).unwrap();

    assert_eq!(circuit, expected_asm);
}

#[test]
fn test_arithmetic() {
    let asm = "@ noname.0.1.0

DoubleGeneric<1>
DoubleGeneric<1,1,-1>
DoubleGeneric<1,0,0,0,-2>
DoubleGeneric<1,-1>
(0,0) -> (1,1)
(1,2) -> (3,1)
(2,0) -> (3,0)
";
    test_file("arithmetic.no", asm);
}

#[test]
fn test_public_output() {
    let asm = "@ noname.0.1.0

DoubleGeneric<1>
DoubleGeneric<1>
DoubleGeneric<1,1,-1>
DoubleGeneric<1,0,0,0,-2>
DoubleGeneric<1,-1>
DoubleGeneric<1,0,-1,0,6>
(0,0) -> (2,1)
(2,2) -> (4,1) -> (5,0)
(3,0) -> (4,0)
";

    test_file("public_output.no", asm);
}

#[test]
fn test_poseidon() {
    let asm = "@ noname.0.1.0

DoubleGeneric<1>
c0=-7792942617772573725741879823703654500237496169155240735183726605099215774906
c1=-12064580110929698653240246574448046290252884192706638916253023953560874107988
c2=-11917335272903734152373661187169745042425461659790767624711412916967075807667
c3=-3731304072199566103171469806803284903365154601287512832324400417928510202977
c4=9054264347380455706540423067244764093107767235485930776517975315876127782582
c5=-2508935187882455694939176059280069137836796157461213077227301028806357773449
c6=-13697021518511787686253351755320145229543125885816346402870494237739111842843
c7=10861916012597714684433535077722887124099023163589869801449218212493070551767
c8=-10350368786058447668580217773185588935099325714445585345388149561403537526198
c9=-13116605855130404579329427245366486913902734252883804253374647582502378623726
c10=-13776165390073083238186891337723331261349016957782089173102544333989100428045
c11=-13459526350449455208410031108267224177473239692289154827027559657901460004586
c12=-9908219629345985367758441581173251013520401282651598748153452794510143690185
c13=4720101937153217036737330058775388037616286510783561045464678919473230044408
c14=10226318327254973427513859412126640040910264416718766418164893837597674300190
Poseidon<c0,c1,c2,c3,c4,c5,c6,c7,c8,c9,c10,c11,c12,c13,c14>
c0=-8069266178199830448972230392936839687503211843639592826513414734203935791518
c1=7178475685651744631172532830973371642652029385893667810726019303466125436953
c2=1996970955918516145107673266490486752153434673064635795711751450164177339618
c3=-13742476392894891390963326106415079641880741683031407140614245947127462957707
c4=-3287725347776349282068482036367697912040723582469209991538019378287640419639
c5=13842611741937412200312851417353455040950878279339067816479233688850376089318
c6=1383799642177300432144836486981606294838630135265094078921115713566691160459
c7=1135532281155277588005319334542025976079676424839948500020664227027300010929
c8=4384117336930380014868572224801371377488688194169758696438185377724744869360
c9=-7222444733618778784083863916271606053938452034858207244062672738169475436688
c10=676128913284806802699862508051022306366147359505124346651466289788974059668
c11=-3761410969730630123225965202342793076550404989096552382536252017856867041130
c12=10402240124664763733060094237696964473609580414190944671778761753887884341073
c13=11918307118590866200687906627767559273324023585642003803337447146531313172441
c14=-12052345054933387831706453748635314609181341144311183806176673496038670993036
Poseidon<c0,c1,c2,c3,c4,c5,c6,c7,c8,c9,c10,c11,c12,c13,c14>
c0=-5129419610296307186018247795475651257864673351720263135555640986230754405527
c1=4285193711150023248690088154344086684336247475445482883105661485741762600154
c2=-9814817865939626451836595586308025713140121891749294344375726028524814391725
c3=5515589673266504033533906836494002702866463791762187140099560583198974233395
c4=11830435563729472715615302060564876527985621376031612798386367965451821182352
c5=7510711479224915247011074129666445216001563200717943545636462819681638560128
c6=-4253179107421325915801242625440146906812928256644190498344348185616579896893
c7=-1586367242355264202329320588080593904448753902246663527935254570785043519809
c8=-7341234123134514614725912297800963174729560695521841760474185286305554527624
c9=-9013962245938143446583338644357189628204034665404554712556641526642043623580
c10=8495813630060004961768092461554180468161254914257386012937942498774724649553
c11=-1423061628799286653887415787445068269418395520940601873536749456408405781876
c12=-13769540658378649596134940851556341260276801446867641601287422214659104733352
c13=-12783241954633376596101641054897467712221650768928755778847361801798367249467
c14=10529167793600778056702353412758954281652843049850979705476598375597148191979
Poseidon<c0,c1,c2,c3,c4,c5,c6,c7,c8,c9,c10,c11,c12,c13,c14>
c0=721141070179074082553302896292167103755384741083338957818644728290501449040
c1=-6903613323372814831958655873799602080263940728823299403481125766161818718296
c2=-1879768206087059003003874089646910814996041790458959568418362547100921444022
c3=3880429241956357176819112098792744584376727450211873998699580893624868748961
c4=-11560925183806111232630238186205227461780038957331863588866465196213633974714
c5=6256814421247770895467770393029354017922744712896100913895513234184920631289
c6=2942627347777337187690939671601251987500285937340386328746818861972711408579
c7=-4916367371564761575344118123681902161553955158698014402128503333452558684940
c8=14401457902976567713827506689641442844921449636054278900045849050301331732143
c9=-8777389431943642405150546415238076705670432128051712363547085970138128499610
c10=-4891526115471604130568335823310254625188956687856973951087567640668240340156
c11=11257913009612703357266904349759250619633397075667824800196659858304604714965
c12=-6719863387344623106693674790661824269337298610380153818912888727233036621091
c13=9152163378317846541430311327336774331416267016980485920222768197583559318682
c14=13906695403538884432896105059360907560653506400343268230130536740148070289175
Poseidon<c0,c1,c2,c3,c4,c5,c6,c7,c8,c9,c10,c11,c12,c13,c14>
c0=7220714562509721437034241786731185291972496952091254931195414855962344025067
c1=-1339155003425237458683883450190631085183719112574006237749117074757077938410
c2=13288465747219756218882697408422850918209170830515545272152965967042670763153
c3=8251343892709140154567051772980662609566359215743613773155065627504813327653
c4=-6912783944226877247725801624678344303118743918006851959820379603017058751247
c5=13560937766273321037807329177749403409731524715067067740487246745322577571823
c6=-7295503700369814305630187116886618942810159132006989551922337577353162222297
c7=-6468935346004875428258285910026425708351309488031424141028503183280364543446
c8=13676501958531751140966255121288182631772843001727158043704693838707387130095
c9=5680310394102577950568930199056707827608275306479994663197187031893244826674
c10=-3822661858422882216702353489100419553315720726600500365074857278843724340339
c11=-6288768280827432070863151759797733381760312117081798476450328334515742953661
c12=-5846610904241536684470907395412528785850376612058573084881107322853245093555
c13=-4798248296088692903835622591515512020953727844661123199989776933361788762229
c14=5782097512368226173095183217893826020351125522160843964147125728530147423065
Poseidon<c0,c1,c2,c3,c4,c5,c6,c7,c8,c9,c10,c11,c12,c13,c14>
c0=13540762114500083869920564649399977644344247485313990448129838910231204868111
c1=-8526384575000237518365198548338963685531251496503153313967052693628827716355
c2=7742664118615900772129122541139124149525273579639574972380600206383923500701
c3=1109643801053963021778418773196543643970146666329661268825691230294798976318
c4=-12367358388511995012771682559443277072410551407554798936679239768108066406497
c5=-14309507629106619797652460333341870755337827022595527245167565469502845837971
c6=-11867636451516376206403528286886249223805483014927167892962655499648404424446
c7=-2771754197592311297389970258246280171388317688846536891924849186780436921672
c8=4382756253392449071896813428140986330161215829425086284611219278674857536001
c9=13934033814940585315406666445960471293638427404971553891617533231178815348902
c10=-1893109576349295541118328023772746529399913304278712631909427240078921457216
c11=-31951905630455479401769575637014370821043461930916981333474279489926386946
c12=-4128006672362688705728288157277389197978921222495265437852678633415003707956
c13=7969535238488580655870884015145760954416088335296905520306227531221721881868
c14=7690547696740080985104189563436871930607055124031711216224219523236060212249
Poseidon<c0,c1,c2,c3,c4,c5,c6,c7,c8,c9,c10,c11,c12,c13,c14>
c0=9712576468091272384496248353414290908377825697488757134833205246106605867289
c1=12148698031438398980683630141370402088785182722473169207262735228500190477924
c2=14359657643133476969781351728574842164124292705609900285041476162075031948227
c3=-5384182343956981580754753451136196949940827484217274654979641045304615194867
c4=4184634822776323233231956802962638484057536837393405750680645555481330909086
c5=-12698510404143276730130707463133783848931970877956481076064881041848751137850
c6=11001863048692031559800673473526311616702863826063550559568315794438941516621
c7=4702354107983530219070178410740869035350641284373933887080161024348425080464
c8=-5196341801795984617099003940741633052642849756058119090060418281344988128724
c9=-277495793170597385722872755630237417502878724148231622909154332070873111571
c10=3568312993091537758218792253361873752799472566055209125947589819564395417072
c11=1819755756343439646550062754332039103654718693246396323207323333948654200950
c12=5372129954699791301953948907349887257752247843844511069896766784624930478273
c13=-11435865621294102935287130401621826486891135305460521000220697582811476154257
c14=-3170916967011426690733681340258828177391909253163883280754547797505758747278
Poseidon<c0,c1,c2,c3,c4,c5,c6,c7,c8,c9,c10,c11,c12,c13,c14>
c0=-3597630303170307106758507945845711207277601324928859129951375891712080472355
c1=-8851297364045281559006587132026599995882659114951067137057061560053093675493
c2=8063283381910110762785892100479219642751540456251198202214433355775540036851
c3=4393613870462297385565277757207010824900723217720226130342463666351557475823
c4=9874972555132910032057499689351411450892722671352476280351715757363137891038
c5=-5357095834999146504453308100575110652117373799506325545953329252352724725469
c6=-11224648938191772996425227636620698378520108518046769683657901808480009419267
c7=2350345015303336966039836492267992193191479606566494799781846958620636621159
c8=-1192814426538837715209735670315488997775989509958935204802379226815344225321
c9=6584607987789185408123601849106260907671314994378225066806060862710814193906
c10=609759108847171587253578490536519506369136135254150754300671591987320319770
c11=-512834723363446745818404001261368647330111294465118847287962741820164597254
c12=-12931357397677278191953829801926271055075863517686856074236925660885645175034
c13=-11396749016174352766825778080592581162440852215310686644768354045446008291174
c14=-8533826811334294326413713785156260024768334452894352881095843926268554580139
Poseidon<c0,c1,c2,c3,c4,c5,c6,c7,c8,c9,c10,c11,c12,c13,c14>
c0=-9174714390478363392712455285397511157825535886339064186330108579356480036482
c1=-4349418470516886035134907887986850630082924634193823181964877296482736463357
c2=11040972566103463398651864390163813377135738019556270484707889323659789290225
c3=5189242080957784038860188184443287562488963023922086723850863987437818393811
c4=1435203288979376557721239239445613396009633263160237764653161500252258220144
c5=13066591163578079667911016543985168493088721636164837520689376346534152547210
c6=-11602120902315449437744535787021111180734634434483535908464174274638714798995
c7=-6808388947079376955764717119784701423999372293587591650666181761678234429989
c8=1061056418502836172283188490483332922126033656372467737207927075184389487061
c9=10241738906190857416046229928455551829189196941239601756375665129874835232299
c10=-1139988976911203743600337578961977642379398785567622456602725347778422265922
c11=-10127867319455374594395100527268058916668914002701011028869014138878389893197
c12=7983688435214640842673294735439196010654951226956101271763849527529940619307
c13=-11880093651527241206966990695305300064217595711588828897891767121200399358771
c14=-4475951484172812026377008160380794106937421048553358562596096229539722687575
Poseidon<c0,c1,c2,c3,c4,c5,c6,c7,c8,c9,c10,c11,c12,c13,c14>
c0=-3195821139967252944634120521155259549052070031936823201359435726313031347110
c1=-2906516933044382695760626363222159713788367335017364650991668051370711522802
c2=-4970971820232933645501027653150149183313847167658448994089720692529864784329
c3=-2269765212050260445216719533435889650547039732924821782012542163624005216532
c4=10480026985951498884090911619636977502506079971893083605102044931823547311729
c5=-7821391008736041800775623421210703092195301927271243290132593430792432166941
c6=1564862894215434177641156287699106659379648851457681469848362532131406827573
c7=13247162472821152334486419054854847522301612781818744556576865965657773174584
c8=8673615954922496961704442777870253767001276027366984739283715623634850885984
c9=2794525076937490807476666942602262298677291735723129868457629508555429470085
c10=4656175953888995612264371467596648522808911819700660048695373348629527757049
c11=-5726448071471388537449178959570415030873434562837334551975766919175351153008
c12=1878392460078272317716114458784636517603142716091316893054365153068227117145
c13=2370412714505757731457251173604396662292063533194555369091306667486647634097
c14=-11538237447458858925126106326777785074695738719613133126800686952369815257061
Poseidon<c0,c1,c2,c3,c4,c5,c6,c7,c8,c9,c10,c11,c12,c13,c14>
c0=-3078885667430882341780804543563928693778823239167746701569112663181193337143
c1=11361209360311194794795494027949518465383235799633128250259863567683341091323
c2=-14034763488610227620815366401073256891460885779828021904842345148790557641768
c3=12957012022018304419868287033513141736995211906682903915897515954290678373899
c4=-11819132761878364289881773806843117667559028774579797238152626652286337080037
c5=-5618803223956816084604439484929241718344912624318409560373493984580662140434
c6=1607741027962933685476527275858938699728586794398382348454736018784568853937
c7=2611953825405141009309433982109911976923326848135736099261873796908057448476
c8=7372230383134982628913227482618052530364724821976589156840317933676130378411
c9=-8744415550827836235050011128401962010863301730510900252893979774032410811766
c10=4678361398979174017885631008335559529633853759463947250620930343087749944307
c11=-1771559675130577479890458980417855037612306804942524550497117377154843604743
c12=6361981813552614697928697527332318530502852015189048838072565811230204474643
c13=13815234633287489023151647353581705241145927054858922281829444557905946323248
c14=10888828634279127981352133512429657747610298502219125571406085952954136470354
Poseidon<c0,c1,c2,c3,c4,c5,c6,c7,c8,c9,c10,c11,c12,c13,c14>
DoubleGeneric<>
DoubleGeneric<1,-1>
(0,0) -> (13,1)
(12,0) -> (13,0)
";

    test_file("poseidon.no", asm);
}
