use anyhow::Result;

const INPUT: &str = "7678999878989655432345789645679998765678998765434567897854323489543467898999785456789123989109765458
6567898767878943101234678923678987654599987533323468976543212379954578967898654398993299879998754387
5428988654567893212645699213567898543689876321013456897654323567895699656799543237894988659876543235
6219876532178954324596988901289999698799965434565567998865439878976789545678932146999876545995432124
5301988631079966546689877892397898799899876567677678989876545689989897234999943239898985432987654265
6212987432367897656899756789456789989943987878998789878987659792196976546899865498767994321398754376
5429876543456789897998767896569898978932198989949896569999798943235989756798976987656789520999866789
6534989654678898999239878987698987567893239996823987698898987894476799868987897896547897639898997894
7676798798799967894345989399897696457894547895612998987657896789987897979986789987535679798767989923
8787899999895459965456895499987545348987656954109899996532345996598965499765678996547899987955679434
9898998798986578977567896989876432167899767963298789987831234897679879987654567987698929896434598945
9949987696598789898978999876989643456789878974987679876542546789889999998543479998999019765323597896
8939998985439896789989389965597656587898999989876545989753657895999989899697568919998998943210126789
7998999874321965889991278954498768998987999899976432399864798934989876789987678924987987895434335991
6877999965439654567892469893349879019876789789876541989878999329876545678998789439876896796545449890
5456889879998963456953998792123989929965456699987799876989987912983534767999896598965345689987659789
5345678998787999769899877689934599898766344578999989545697896899892123457899987987421236799998998678
1234589987656787998787654577895789789654213799098765433456895698789012456999699876434349988999987567
2455689898546876987665343456789895698766439893129876212356924997679133477897578987646498877899999456
3576798765435455696543212567899964569987898993298865101248939876568949588926496798797987765698976567
4799897654321234987854343456999653456798987989987654312346795325459998999212345699998986564567899688
6989998767433345699765454567898799999969876678999985323489893210298987992101296789029897423456998789
8979879876544476789876565778919987878954965567899876544678954321987676789212989893298775212347999893
9769865989699988895987986789929876565969854349965988965989875439896565678939878965997654303567895932
8949876798987899954598987899899975444898765456894999876898986698765434567898767899876543212347894541
6732987987896789543459998987698765323789878677923899988987897987854324678987656999989754323456943210
5421299896785678952349879999549886212678989989765789199876759876544215799999545698999865534568964721
7532398765654679893499767897632998343567893598986891098765345975432106789987234997964987665778965673
9678998544263456789988959789543987657678932467897932129954234596543312899876129876853498786899876789
8989876432102345679876434678954598767899743478998943498764345987986324678944098765432129897999998996
7899987653212456789954323499765679898987654589129654569895456798875435899432129874321012979998999765
6789998764343568899983212389876789999598765699939765678987667899986545986543298985435433567897898654
5698999889654689969874301278987899995439876799898996789998789967987676897684987899876574679946999543
4567894998788797645965212367998978989567988899787789898999895456798789999895996212987989789035698912
3459943219899895434596776456789765878979899987645678987689954345679896798999854301298999892124567901
2588954323999954323987886567898954569898789895434567998567893234699945987998765412569321943235678912
1467975754798765102498997688967932456789698765423478899455789124989239886569885323457939654345679324
2356899765679843212349998799456891348995569543213456789324678939878998776457995434567898965567789535
4567899879789764334467999890345989969544398754302398893213999998767987654349876776699957896788998745
7679943989999878876578987921235679898931239965614689965354899989654398768124997898789546789899987656
8789432199899989987679765434356898777999459877323567896765789878965459878236898949895435899987898769
9996431098788991098789879865468999656788968986545978987976899769879878989356999434976567978976989898
6797643989657989199894989976789987547897899898767989898988998656989989198767898921297678967965678997
5989859764546778987953299987898798623456798769878998789999987545699993239878957890998989456894567986
4578998943436567996592109798987643214567989657989987667999987636798954569989346989879594345993479875
3459976832323456789489297659298754323579876545699876557989999947997895678994219878965432256989599854
1569765321012367892378989432129898734567988435689764345678998799896989789875699767895421019878989763
2979876465433458921269878993234987655689876324997653256789989654765678999986987656789532198767978932
9899997598545769210359767989345698978789995436789767967999978943234568989999876545679673299956767891
7678998987656898521298655678956789989890987545679978979998767894345679878998765434568984987845656789
5467869998769987439987834567997891299991987676789989999987656789459894567899875323479895996434345679
4328956999878976598765423456789920198989998789897594987654345679767893479999983212998789875321234589
3212347899999987689896514567895439987878999899969423498543234578978942346789765929877678995434345699
4323456789998998799987625678976598965456899998754312987654547899989321457899879898765534987646656789
5436567998987899989699876789987987854345799879876533498987656789795452567954999789854424987657898896
9597778987656999879545987899898996543236878965987645569998768995696563467892987689943215598768959934
8989899876545698768996798998759897652123467894598786689999879654989694598993995578994103469878948923
7678956989656987656989899987545789643014989912989897899898989769979989679989894469989912456989237894
6567899998768996545678956795437699654423458909878969998797899898768778989977673234767893567890146789
5454578999899998656989349989326578967434567898769459997656999987855667899866582123456789678921237993
5323457898988998767892198975412468998658998987658998878546689876543556789954321014569899989743678932
5434578987677789878989987654301656789767899498767897651234579994312347898765432785678999998654568941
8656789876435678989678999765212345699878901399988999530145678987523556789879545676789898999875678932
8798999754323699094567899874323456899999213989999998921456789797687678999988657887896787893986989543
9899898765214589123789998765474587998765439778899876542347895698798789999899767998965345932397899655
8946789874345678934690989987565678999876598656789987673656954329899898898789898979873234991098998976
7657894995456789545791976798786789989989987545791298784967899212945997787678999965432125789199687897
9767923987877897656999875899897899878998996434690249899878998909896985678567899876553239899987545699
9898934998988998987899987899998999767987654323589356976989876898799864323456789998674999989995432378
3939999879399899598978998998789998656798895212679467895499965536567973212345892139799889578976653467
2123989765456789799767899887699987545989987825678978954349873213459854323456999012998778989987854578
1034979876789894987656789765569895239875498434569989767459654323498765456567897999875667899998769989
3249867987899965986545678954498764345984399565878999878598776434789896587689966789943458999879898999
4398756798959896965434989543359895459865987676789999998679986565699987678797655698965567896567987678
5987545789349789874323478921239876567978998787899989598789997676789998889893234987896689965439876567
6798435678998678955412567890124989878989799898998879329899898789897899997992109986999791986323965456
8999323699987589432101678921265696989997678999997568912998759897996987656789298975698910987439854345
9998434598765478943312989892376965399876546798775467893498942956889876545678987764567891296598763234
6987545789876569765529899789499894236995435987655345679987621345678965434786796543456799397998521065
5698656795998978997698765678998789945989412396543238789296549496789876212345987432345678989997432156
4569767894349989989899876789987687899875324987652146790197698989997984101456798543456789979876543234
7695979943256997878999989899876546798765459876543234892398797678976543219597987654767899865989754546
8983598954569876557998998913985434569878969987656345789989986567897655998989998866978998754599867656
9672567895698987445987897654954323678989978998765456999879876458989769897778999877899999863212998767
8543479976987894323456789799899939899497989549877667898765964345678979786667899988989987654201239879
7656567899996789434569899988767898954345798767998788987654321234589998654557898999678996543212378989
8767678998987896545678999875456787943234679878999899876543210156799987543238967976579998754323567898
9878789987698939876789398754346896795345799989989942987654921234578975432123459985458999895454578967
2989999896579920987996209543212345689567989899875321298769832345689986543234567894367899986895789456
3499998755465891298965498932101234567978978789965433349987655466789987654355678965456989998999892345
9567897645234789349996987893323346788989865667899564559998776567894298765696989796567978999998921234
8978965432145679959889896894894487899998754345978975698989897678992199896787897689679867899867890123
7899876541034567898775765789765698910987543234567896987676998989989987979898976578998956989656989934
6987998643123456789654324899876789991299654345678999876545789999879876569969323488987645678945679895
5476789654234569898775013678989898989399865476789998964335679986567965498653212346986534799134998799
4345698765545878959898723459994967979988976597998997643212398765457894329965623678997656789029898678
3236989876676789543997634678943459869877997689367899854323459854346965939876784569789867997998767567
2135678987798994312498545689642399759866859789245699978754667965457899845989765679679878956999543458
3446889398919865324987656796530987649754349890134589989765778976579998756799897789545989349898742455
4587993209101965435698787895421298432101245932375678999876899987678999867899998996432191298765431234";

// const INPUT: &str = "2199943210
// 3987894921
// 9856789892
// 8767896789
// 9899965678";

fn is_minima(index: (usize, usize), grid: &Vec<Vec<u32>>) -> bool {
    let increments: [[isize; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];

    let point = grid[index.1][index.0];

    for increment in increments.iter() {
        let left_incr = index.0 as isize + increment[0];
        if increment[0] != 0 && left_incr >= 0 && left_incr < grid[0].len() as isize {
            if point >= grid[index.1][left_incr as usize] {
                return false;
            }
        }

        let right_incr = index.1 as isize + increment[1];
        if increment[1] != 0 && right_incr >= 0 && right_incr < grid.len() as isize {
            if point >= grid[right_incr as usize][index.0] {
                return false;
            }
        }
    }

    true
}

fn mark_basins(grid: &mut Vec<Vec<u32>>) {
    let increments: [[isize; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];

    // Go through the grid
    // Look at all the neighbours
    // If not nine and no neighbouring basins, mark with basin index
    // Else mark with existing index
    let mut basin_index = 10;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 9 {
                continue;
            }
            let mut found_basin = false;
            for increment in increments.iter() {
                let left_incr = j as isize + increment[0];
                if increment[0] != 0 && left_incr >= 0 && left_incr < grid[0].len() as isize {
                    if grid[i][left_incr as usize] >= 10 {
                        if grid[i][left_incr as usize] < grid[i][j] || grid[i][j] < 10 {
                            grid[i][j] = grid[i][left_incr as usize];
                            found_basin = true;
                            continue;
                        }
                        found_basin = true;
                    }
                }

                let right_incr = i as isize + increment[1];

                if increment[1] != 0 && right_incr >= 0 && right_incr < grid.len() as isize {
                    if grid[right_incr as usize][j] >= 10 {
                        if grid[right_incr as usize][j] < grid[i][j] || grid[i][j] < 10 {
                            grid[i][j] = grid[right_incr as usize][j];
                            found_basin = true;
                            continue;
                        }
                        found_basin = true;
                    }
                }
            }

            if !found_basin {
                grid[i][j] = basin_index;
                basin_index += 1;
            }
        }
    }

    let mut basin_index = 10;
    for (i, mut x) in grid.iter_mut().enumerate() {
        for (j, mut y) in x.iter_mut().enumerate() {}
    }
}

pub fn day_09() -> Result<()> {
    let mut lines: Vec<Vec<u32>> = INPUT
        .split("\n")
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();

    let answer: u32 = lines
        .iter()
        .enumerate()
        .map(|(i, x)| {
            x.iter()
                .enumerate()
                .filter(|(j, y)| is_minima((*j, i), &lines))
                .map(|(j, y)| y + 1)
                .sum::<u32>()
        })
        .sum();

    println!("{:?}", answer);

    for i in 0..50 {
        mark_basins(&mut lines);
        mark_basins(&mut lines);
        mark_basins(&mut lines);
    }

    let mut answer: Vec<u32> = lines
        .iter()
        .map(|x| x.iter())
        .flatten()
        .cloned()
        .filter(|x| *x != 9)
        .collect();

    answer.sort();

    let mut counts = vec![];

    for i in answer[0]..(*answer.last().unwrap() + 1) {
        counts.push(answer.iter().filter(|x| **x == i).count())
    }

    counts.sort();

    let answer = counts.iter().rev().take(3).fold(1, |acc, x| x * acc);

    println!("{:?}", answer);

    Ok(())
}
