// Sonic
//
// Fast, lightweight and schema-less search backend
// Copyright: 2019, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

pub static STOPWORDS_CES: &[&str] = &[
    "a",
    "aby",
    "ahoj",
    "aj",
    "ale",
    "anebo",
    "ani",
    "aniž",
    "ano",
    "asi",
    "aspoåˆ",
    "aspoň",
    "atd",
    "atp",
    "az",
    "aäkoli",
    "ačkoli",
    "až",
    "bez",
    "beze",
    "blã\u{AD}zko",
    "blízko",
    "bohuå¾el",
    "bohužel",
    "brzo",
    "bude",
    "budem",
    "budeme",
    "budes",
    "budete",
    "budeå¡",
    "budeš",
    "budou",
    "budu",
    "by",
    "byl",
    "byla",
    "byli",
    "bylo",
    "byly",
    "bys",
    "byt",
    "bä›hem",
    "být",
    "během",
    "chce",
    "chceme",
    "chcete",
    "chceå¡",
    "chceš",
    "chci",
    "chtã\u{AD}t",
    "chtä›jã\u{AD}",
    "chtít",
    "chtějí",
    "chut'",
    "chuti",
    "ci",
    "clanek",
    "clanku",
    "clanky",
    "co",
    "coz",
    "což",
    "cz",
    "daleko",
    "dalsi",
    "další",
    "den",
    "deset",
    "design",
    "devatenáct",
    "devatenã¡ct",
    "devä›t",
    "devět",
    "dnes",
    "do",
    "dobrã½",
    "dobrý",
    "docela",
    "dva",
    "dvacet",
    "dvanáct",
    "dvanã¡ct",
    "dvä›",
    "dvě",
    "dál",
    "dále",
    "dã¡l",
    "dã¡le",
    "dä›kovat",
    "dä›kujeme",
    "dä›kuji",
    "děkovat",
    "děkujeme",
    "děkuji",
    "email",
    "ho",
    "hodnä›",
    "hodně",
    "i",
    "jak",
    "jakmile",
    "jako",
    "jakož",
    "jde",
    "je",
    "jeden",
    "jedenáct",
    "jedenã¡ct",
    "jedna",
    "jedno",
    "jednou",
    "jedou",
    "jeho",
    "jehož",
    "jej",
    "jeji",
    "jejich",
    "jejã\u{AD}",
    "její",
    "jelikož",
    "jemu",
    "jen",
    "jenom",
    "jenž",
    "jeste",
    "jestli",
    "jestliå¾e",
    "jestliže",
    "jeå¡tä›",
    "ještě",
    "jež",
    "ji",
    "jich",
    "jimi",
    "jinak",
    "jine",
    "jiné",
    "jiz",
    "již",
    "jsem",
    "jses",
    "jseš",
    "jsi",
    "jsme",
    "jsou",
    "jste",
    "já",
    "jã¡",
    "jã\u{AD}",
    "jã\u{AD}m",
    "jí",
    "jím",
    "jíž",
    "jšte",
    "k",
    "kam",
    "každý",
    "kde",
    "kdo",
    "kdy",
    "kdyz",
    "kdyå¾",
    "když",
    "ke",
    "kolik",
    "kromä›",
    "kromě",
    "ktera",
    "ktere",
    "kteri",
    "kterou",
    "ktery",
    "která",
    "kterã¡",
    "kterã©",
    "kterã½",
    "které",
    "který",
    "kteå™ã\u{AD}",
    "kteři",
    "kteří",
    "ku",
    "kvå¯li",
    "kvůli",
    "ma",
    "majã\u{AD}",
    "mají",
    "mate",
    "me",
    "mezi",
    "mi",
    "mit",
    "mne",
    "mnou",
    "mnä›",
    "mně",
    "moc",
    "mohl",
    "mohou",
    "moje",
    "moji",
    "moå¾nã¡",
    "možná",
    "muj",
    "musã\u{AD}",
    "musí",
    "muze",
    "my",
    "má",
    "málo",
    "mám",
    "máme",
    "máte",
    "máš",
    "mã¡",
    "mã¡lo",
    "mã¡m",
    "mã¡me",
    "mã¡te",
    "mã¡å¡",
    "mã©",
    "mã\u{AD}",
    "mã\u{AD}t",
    "mä›",
    "må¯j",
    "må¯å¾e",
    "mé",
    "mí",
    "mít",
    "mě",
    "můj",
    "může",
    "na",
    "nad",
    "nade",
    "nam",
    "napiste",
    "napište",
    "naproti",
    "nas",
    "nasi",
    "naå¡e",
    "naå¡i",
    "načež",
    "naše",
    "naši",
    "ne",
    "nebo",
    "nebyl",
    "nebyla",
    "nebyli",
    "nebyly",
    "nechť",
    "nedä›lajã\u{AD}",
    "nedä›lã¡",
    "nedä›lã¡m",
    "nedä›lã¡me",
    "nedä›lã¡te",
    "nedä›lã¡å¡",
    "nedělají",
    "nedělá",
    "nedělám",
    "neděláme",
    "neděláte",
    "neděláš",
    "neg",
    "nejsi",
    "nejsou",
    "nemajã\u{AD}",
    "nemají",
    "nemáme",
    "nemáte",
    "nemã¡me",
    "nemã¡te",
    "nemä›l",
    "neměl",
    "neni",
    "nenã\u{AD}",
    "není",
    "nestaäã\u{AD}",
    "nestačí",
    "nevadã\u{AD}",
    "nevadí",
    "nez",
    "neå¾",
    "než",
    "nic",
    "nich",
    "nimi",
    "nove",
    "novy",
    "nové",
    "nový",
    "nula",
    "ná",
    "nám",
    "námi",
    "nás",
    "náš",
    "nã¡m",
    "nã¡mi",
    "nã¡s",
    "nã¡å¡",
    "nã\u{AD}m",
    "nä›",
    "nä›co",
    "nä›jak",
    "nä›kde",
    "nä›kdo",
    "nä›mu",
    "ní",
    "ním",
    "ně",
    "něco",
    "nějak",
    "někde",
    "někdo",
    "němu",
    "němuž",
    "o",
    "od",
    "ode",
    "on",
    "ona",
    "oni",
    "ono",
    "ony",
    "osm",
    "osmnáct",
    "osmnã¡ct",
    "pak",
    "patnáct",
    "patnã¡ct",
    "po",
    "pod",
    "podle",
    "pokud",
    "potom",
    "pouze",
    "pozdä›",
    "pozdě",
    "poå™ã¡d",
    "pořád",
    "prave",
    "pravé",
    "pred",
    "pres",
    "pri",
    "pro",
    "proc",
    "prostä›",
    "prostě",
    "prosã\u{AD}m",
    "prosím",
    "proti",
    "proto",
    "protoze",
    "protoå¾e",
    "protože",
    "proä",
    "proč",
    "prvni",
    "první",
    "práve",
    "pta",
    "pä›t",
    "på™ed",
    "på™es",
    "på™ese",
    "pět",
    "před",
    "přede",
    "přes",
    "přese",
    "při",
    "přičemž",
    "re",
    "rovnä›",
    "rovně",
    "s",
    "se",
    "sedm",
    "sedmnáct",
    "sedmnã¡ct",
    "si",
    "sice",
    "skoro",
    "smã\u{AD}",
    "smä›jã\u{AD}",
    "smí",
    "smějí",
    "snad",
    "spolu",
    "sta",
    "sto",
    "strana",
    "stã©",
    "sté",
    "sve",
    "svych",
    "svym",
    "svymi",
    "své",
    "svých",
    "svým",
    "svými",
    "svůj",
    "ta",
    "tady",
    "tak",
    "take",
    "takhle",
    "taky",
    "takze",
    "také",
    "takže",
    "tam",
    "tamhle",
    "tamhleto",
    "tamto",
    "tato",
    "te",
    "tebe",
    "tebou",
    "ted'",
    "tedy",
    "tema",
    "ten",
    "tento",
    "teto",
    "ti",
    "tim",
    "timto",
    "tipy",
    "tisã\u{AD}c",
    "tisã\u{AD}ce",
    "tisíc",
    "tisíce",
    "to",
    "tobä›",
    "tobě",
    "tohle",
    "toho",
    "tohoto",
    "tom",
    "tomto",
    "tomu",
    "tomuto",
    "toto",
    "troå¡ku",
    "trošku",
    "tu",
    "tuto",
    "tvoje",
    "tvá",
    "tvã¡",
    "tvã©",
    "två¯j",
    "tvé",
    "tvůj",
    "ty",
    "tyto",
    "tä›",
    "tå™eba",
    "tå™i",
    "tå™inã¡ct",
    "téma",
    "této",
    "tím",
    "tímto",
    "tě",
    "těm",
    "těma",
    "těmu",
    "třeba",
    "tři",
    "třináct",
    "u",
    "uräitä›",
    "určitě",
    "uz",
    "uå¾",
    "už",
    "v",
    "vam",
    "vas",
    "vase",
    "vaå¡e",
    "vaå¡i",
    "vaše",
    "vaši",
    "ve",
    "vedle",
    "veäer",
    "večer",
    "vice",
    "vlastnä›",
    "vlastně",
    "vsak",
    "vy",
    "vám",
    "vámi",
    "vás",
    "váš",
    "vã¡m",
    "vã¡mi",
    "vã¡s",
    "vã¡å¡",
    "vå¡echno",
    "vå¡ichni",
    "vå¯bec",
    "vå¾dy",
    "více",
    "však",
    "všechen",
    "všechno",
    "všichni",
    "vůbec",
    "vždy",
    "z",
    "za",
    "zatã\u{AD}mco",
    "zatímco",
    "zaä",
    "zač",
    "zda",
    "zde",
    "ze",
    "zpet",
    "zpravy",
    "zprávy",
    "zpět",
    "äau",
    "ätrnã¡ct",
    "ätyå™i",
    "å¡est",
    "å¡estnã¡ct",
    "å¾e",
    "čau",
    "či",
    "článek",
    "článku",
    "články",
    "čtrnáct",
    "čtyři",
    "šest",
    "šestnáct",
    "že",
];
