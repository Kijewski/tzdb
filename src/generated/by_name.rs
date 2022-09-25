use core::mem::transmute;

use tz::TimeZoneRef;

#[cfg(feature = "binary")]
use super::raw_tzdata;
use super::tzdata;

#[derive(Clone, Copy)]
#[repr(u16)]
enum Index {
    V0 = 0,
    V1 = 1,
    V2 = 2,
    V3 = 3,
    V4 = 4,
    V5 = 5,
    V6 = 6,
    V7 = 7,
    V8 = 8,
    V9 = 9,
    V10 = 10,
    V11 = 11,
    V12 = 12,
    V13 = 13,
    V14 = 14,
    V15 = 15,
    V16 = 16,
    V17 = 17,
    V18 = 18,
    V19 = 19,
    V20 = 20,
    V21 = 21,
    V22 = 22,
    V23 = 23,
    V24 = 24,
    V25 = 25,
    V26 = 26,
    V27 = 27,
    V28 = 28,
    V29 = 29,
    V30 = 30,
    V31 = 31,
    V32 = 32,
    V33 = 33,
    V34 = 34,
    V35 = 35,
    V36 = 36,
    V37 = 37,
    V38 = 38,
    V39 = 39,
    V40 = 40,
    V41 = 41,
    V42 = 42,
    V43 = 43,
    V44 = 44,
    V45 = 45,
    V46 = 46,
    V47 = 47,
    V48 = 48,
    V49 = 49,
    V50 = 50,
    V51 = 51,
    V52 = 52,
    V53 = 53,
    V54 = 54,
    V55 = 55,
    V56 = 56,
    V57 = 57,
    V58 = 58,
    V59 = 59,
    V60 = 60,
    V61 = 61,
    V62 = 62,
    V63 = 63,
    V64 = 64,
    V65 = 65,
    V66 = 66,
    V67 = 67,
    V68 = 68,
    V69 = 69,
    V70 = 70,
    V71 = 71,
    V72 = 72,
    V73 = 73,
    V74 = 74,
    V75 = 75,
    V76 = 76,
    V77 = 77,
    V78 = 78,
    V79 = 79,
    V80 = 80,
    V81 = 81,
    V82 = 82,
    V83 = 83,
    V84 = 84,
    V85 = 85,
    V86 = 86,
    V87 = 87,
    V88 = 88,
    V89 = 89,
    V90 = 90,
    V91 = 91,
    V92 = 92,
    V93 = 93,
    V94 = 94,
    V95 = 95,
    V96 = 96,
    V97 = 97,
    V98 = 98,
    V99 = 99,
    V100 = 100,
    V101 = 101,
    V102 = 102,
    V103 = 103,
    V104 = 104,
    V105 = 105,
    V106 = 106,
    V107 = 107,
    V108 = 108,
    V109 = 109,
    V110 = 110,
    V111 = 111,
    V112 = 112,
    V113 = 113,
    V114 = 114,
    V115 = 115,
    V116 = 116,
    V117 = 117,
    V118 = 118,
    V119 = 119,
    V120 = 120,
    V121 = 121,
    V122 = 122,
    V123 = 123,
    V124 = 124,
    V125 = 125,
    V126 = 126,
    V127 = 127,
    V128 = 128,
    V129 = 129,
    V130 = 130,
    V131 = 131,
    V132 = 132,
    V133 = 133,
    V134 = 134,
    V135 = 135,
    V136 = 136,
    V137 = 137,
    V138 = 138,
    V139 = 139,
    V140 = 140,
    V141 = 141,
    V142 = 142,
    V143 = 143,
    V144 = 144,
    V145 = 145,
    V146 = 146,
    V147 = 147,
    V148 = 148,
    V149 = 149,
    V150 = 150,
    V151 = 151,
    V152 = 152,
    V153 = 153,
    V154 = 154,
    V155 = 155,
    V156 = 156,
    V157 = 157,
    V158 = 158,
    V159 = 159,
    V160 = 160,
    V161 = 161,
    V162 = 162,
    V163 = 163,
    V164 = 164,
    V165 = 165,
    V166 = 166,
    V167 = 167,
    V168 = 168,
    V169 = 169,
    V170 = 170,
    V171 = 171,
    V172 = 172,
    V173 = 173,
    V174 = 174,
    V175 = 175,
    V176 = 176,
    V177 = 177,
    V178 = 178,
    V179 = 179,
    V180 = 180,
    V181 = 181,
    V182 = 182,
    V183 = 183,
    V184 = 184,
    V185 = 185,
    V186 = 186,
    V187 = 187,
    V188 = 188,
    V189 = 189,
    V190 = 190,
    V191 = 191,
    V192 = 192,
    V193 = 193,
    V194 = 194,
    V195 = 195,
    V196 = 196,
    V197 = 197,
    V198 = 198,
    V199 = 199,
    V200 = 200,
    V201 = 201,
    V202 = 202,
    V203 = 203,
    V204 = 204,
    V205 = 205,
    V206 = 206,
    V207 = 207,
    V208 = 208,
    V209 = 209,
    V210 = 210,
    V211 = 211,
    V212 = 212,
    V213 = 213,
    V214 = 214,
    V215 = 215,
    V216 = 216,
    V217 = 217,
    V218 = 218,
    V219 = 219,
    V220 = 220,
    V221 = 221,
    V222 = 222,
    V223 = 223,
    V224 = 224,
    V225 = 225,
    V226 = 226,
    V227 = 227,
    V228 = 228,
    V229 = 229,
    V230 = 230,
    V231 = 231,
    V232 = 232,
    V233 = 233,
    V234 = 234,
    V235 = 235,
    V236 = 236,
    V237 = 237,
    V238 = 238,
    V239 = 239,
    V240 = 240,
    V241 = 241,
    V242 = 242,
    V243 = 243,
    V244 = 244,
    V245 = 245,
    V246 = 246,
    V247 = 247,
    V248 = 248,
    V249 = 249,
    V250 = 250,
    V251 = 251,
    V252 = 252,
    V253 = 253,
    V254 = 254,
    V255 = 255,
    V256 = 256,
    V257 = 257,
    V258 = 258,
    V259 = 259,
    V260 = 260,
    V261 = 261,
    V262 = 262,
    V263 = 263,
    V264 = 264,
    V265 = 265,
    V266 = 266,
    V267 = 267,
    V268 = 268,
    V269 = 269,
    V270 = 270,
    V271 = 271,
    V272 = 272,
    V273 = 273,
    V274 = 274,
    V275 = 275,
    V276 = 276,
    V277 = 277,
    V278 = 278,
    V279 = 279,
    V280 = 280,
    V281 = 281,
    V282 = 282,
    V283 = 283,
    V284 = 284,
    V285 = 285,
    V286 = 286,
    V287 = 287,
    V288 = 288,
    V289 = 289,
    V290 = 290,
    V291 = 291,
    V292 = 292,
    V293 = 293,
    V294 = 294,
    V295 = 295,
    V296 = 296,
    V297 = 297,
    V298 = 298,
    V299 = 299,
    V300 = 300,
    V301 = 301,
    V302 = 302,
    V303 = 303,
    V304 = 304,
    V305 = 305,
    V306 = 306,
    V307 = 307,
    V308 = 308,
    V309 = 309,
    V310 = 310,
    V311 = 311,
    V312 = 312,
    V313 = 313,
    V314 = 314,
    V315 = 315,
    V316 = 316,
    V317 = 317,
    V318 = 318,
    V319 = 319,
    V320 = 320,
    V321 = 321,
    V322 = 322,
    V323 = 323,
    V324 = 324,
    V325 = 325,
    V326 = 326,
    V327 = 327,
    V328 = 328,
    V329 = 329,
    V330 = 330,
    V331 = 331,
    V332 = 332,
    V333 = 333,
    V334 = 334,
    V335 = 335,
    V336 = 336,
    V337 = 337,
    V338 = 338,
    V339 = 339,
    V340 = 340,
    V341 = 341,
    V342 = 342,
    V343 = 343,
    V344 = 344,
    V345 = 345,
    V346 = 346,
    V347 = 347,
    V348 = 348,
    V349 = 349,
    V350 = 350,
    V351 = 351,
    V352 = 352,
    V353 = 353,
    V354 = 354,
    V355 = 355,
    V356 = 356,
    V357 = 357,
    V358 = 358,
    V359 = 359,
    V360 = 360,
    V361 = 361,
    V362 = 362,
    V363 = 363,
    V364 = 364,
    V365 = 365,
    V366 = 366,
    V367 = 367,
    V368 = 368,
    V369 = 369,
    V370 = 370,
    V371 = 371,
    V372 = 372,
    V373 = 373,
    V374 = 374,
    V375 = 375,
    V376 = 376,
    V377 = 377,
    V378 = 378,
    V379 = 379,
    V380 = 380,
    V381 = 381,
    V382 = 382,
    V383 = 383,
    V384 = 384,
    V385 = 385,
    V386 = 386,
    V387 = 387,
    V388 = 388,
    V389 = 389,
    V390 = 390,
    V391 = 391,
    V392 = 392,
    V393 = 393,
    V394 = 394,
    V395 = 395,
    V396 = 396,
    V397 = 397,
    V398 = 398,
    V399 = 399,
    V400 = 400,
    V401 = 401,
    V402 = 402,
    V403 = 403,
    V404 = 404,
    V405 = 405,
    V406 = 406,
    V407 = 407,
    V408 = 408,
    V409 = 409,
    V410 = 410,
    V411 = 411,
    V412 = 412,
    V413 = 413,
    V414 = 414,
    V415 = 415,
    V416 = 416,
    V417 = 417,
    V418 = 418,
    V419 = 419,
    V420 = 420,
    V421 = 421,
    V422 = 422,
    V423 = 423,
    V424 = 424,
    V425 = 425,
    V426 = 426,
    V427 = 427,
    V428 = 428,
    V429 = 429,
    V430 = 430,
    V431 = 431,
    V432 = 432,
    V433 = 433,
    V434 = 434,
    V435 = 435,
    V436 = 436,
    V437 = 437,
    V438 = 438,
    V439 = 439,
    V440 = 440,
    V441 = 441,
    V442 = 442,
    V443 = 443,
    V444 = 444,
    V445 = 445,
    V446 = 446,
    V447 = 447,
    V448 = 448,
    V449 = 449,
    V450 = 450,
    V451 = 451,
    V452 = 452,
    V453 = 453,
    V454 = 454,
    V455 = 455,
    V456 = 456,
    V457 = 457,
    V458 = 458,
    V459 = 459,
    V460 = 460,
    V461 = 461,
    V462 = 462,
    V463 = 463,
    V464 = 464,
    V465 = 465,
    V466 = 466,
    V467 = 467,
    V468 = 468,
    V469 = 469,
    V470 = 470,
    V471 = 471,
    V472 = 472,
    V473 = 473,
    V474 = 474,
    V475 = 475,
    V476 = 476,
    V477 = 477,
    V478 = 478,
    V479 = 479,
    V480 = 480,
    V481 = 481,
    V482 = 482,
    V483 = 483,
    V484 = 484,
    V485 = 485,
    V486 = 486,
    V487 = 487,
    V488 = 488,
    V489 = 489,
    V490 = 490,
    V491 = 491,
    V492 = 492,
    V493 = 493,
    V494 = 494,
    V495 = 495,
    V496 = 496,
    V497 = 497,
    V498 = 498,
    V499 = 499,
    V500 = 500,
    V501 = 501,
    V502 = 502,
    V503 = 503,
    V504 = 504,
    V505 = 505,
    V506 = 506,
    V507 = 507,
    V508 = 508,
    V509 = 509,
    V510 = 510,
    V511 = 511,
    V512 = 512,
    V513 = 513,
    V514 = 514,
    V515 = 515,
    V516 = 516,
    V517 = 517,
    V518 = 518,
    V519 = 519,
    V520 = 520,
    V521 = 521,
    V522 = 522,
    V523 = 523,
    V524 = 524,
    V525 = 525,
    V526 = 526,
    V527 = 527,
    V528 = 528,
    V529 = 529,
    V530 = 530,
    V531 = 531,
    V532 = 532,
    V533 = 533,
    V534 = 534,
    V535 = 535,
    V536 = 536,
    V537 = 537,
    V538 = 538,
    V539 = 539,
    V540 = 540,
    V541 = 541,
    V542 = 542,
    V543 = 543,
    V544 = 544,
    V545 = 545,
    V546 = 546,
    V547 = 547,
    V548 = 548,
    V549 = 549,
    V550 = 550,
    V551 = 551,
    V552 = 552,
    V553 = 553,
    V554 = 554,
    V555 = 555,
    V556 = 556,
    V557 = 557,
    V558 = 558,
    V559 = 559,
    V560 = 560,
    V561 = 561,
    V562 = 562,
    V563 = 563,
    V564 = 564,
    V565 = 565,
    V566 = 566,
    V567 = 567,
    V568 = 568,
    V569 = 569,
    V570 = 570,
}

const WORDLIST: [Option<Index>; 2324] = [
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V0),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V1),
    None,
    Some(Index::V2),
    None,
    None,
    None,
    None,
    Some(Index::V3),
    Some(Index::V4),
    None,
    Some(Index::V5),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V6),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V7),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V8),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V9),
    None,
    None,
    Some(Index::V10),
    None,
    None,
    None,
    Some(Index::V11),
    None,
    Some(Index::V12),
    Some(Index::V13),
    None,
    None,
    Some(Index::V14),
    Some(Index::V15),
    None,
    None,
    None,
    Some(Index::V16),
    None,
    None,
    None,
    Some(Index::V17),
    None,
    Some(Index::V18),
    Some(Index::V19),
    Some(Index::V20),
    Some(Index::V21),
    None,
    None,
    None,
    Some(Index::V22),
    None,
    Some(Index::V23),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V24),
    None,
    None,
    None,
    Some(Index::V25),
    Some(Index::V26),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V27),
    None,
    None,
    None,
    None,
    Some(Index::V28),
    None,
    Some(Index::V29),
    None,
    Some(Index::V30),
    None,
    None,
    None,
    Some(Index::V31),
    Some(Index::V32),
    Some(Index::V33),
    Some(Index::V34),
    Some(Index::V35),
    None,
    None,
    Some(Index::V36),
    Some(Index::V37),
    Some(Index::V38),
    Some(Index::V39),
    Some(Index::V40),
    Some(Index::V41),
    Some(Index::V42),
    None,
    Some(Index::V43),
    Some(Index::V44),
    None,
    Some(Index::V45),
    Some(Index::V46),
    Some(Index::V47),
    Some(Index::V48),
    Some(Index::V49),
    None,
    Some(Index::V50),
    None,
    None,
    Some(Index::V51),
    None,
    None,
    Some(Index::V52),
    Some(Index::V53),
    Some(Index::V54),
    None,
    None,
    Some(Index::V55),
    None,
    None,
    None,
    Some(Index::V56),
    None,
    None,
    None,
    None,
    Some(Index::V57),
    Some(Index::V58),
    Some(Index::V59),
    None,
    None,
    Some(Index::V60),
    Some(Index::V61),
    Some(Index::V62),
    None,
    Some(Index::V63),
    None,
    Some(Index::V64),
    None,
    Some(Index::V65),
    None,
    Some(Index::V66),
    Some(Index::V67),
    Some(Index::V68),
    Some(Index::V69),
    None,
    Some(Index::V70),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V71),
    None,
    None,
    Some(Index::V72),
    Some(Index::V73),
    Some(Index::V74),
    None,
    Some(Index::V75),
    Some(Index::V76),
    None,
    None,
    Some(Index::V77),
    Some(Index::V78),
    None,
    None,
    None,
    Some(Index::V79),
    None,
    None,
    None,
    None,
    Some(Index::V80),
    None,
    None,
    Some(Index::V81),
    None,
    Some(Index::V82),
    Some(Index::V83),
    None,
    None,
    Some(Index::V84),
    None,
    Some(Index::V85),
    None,
    Some(Index::V86),
    None,
    Some(Index::V87),
    None,
    None,
    None,
    None,
    Some(Index::V88),
    Some(Index::V89),
    Some(Index::V90),
    None,
    None,
    Some(Index::V91),
    None,
    Some(Index::V92),
    None,
    Some(Index::V93),
    None,
    None,
    None,
    None,
    Some(Index::V94),
    Some(Index::V95),
    Some(Index::V96),
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V97),
    Some(Index::V98),
    None,
    Some(Index::V99),
    Some(Index::V100),
    Some(Index::V101),
    Some(Index::V102),
    Some(Index::V103),
    Some(Index::V104),
    Some(Index::V105),
    None,
    Some(Index::V106),
    Some(Index::V107),
    None,
    Some(Index::V108),
    None,
    Some(Index::V109),
    None,
    Some(Index::V110),
    Some(Index::V111),
    Some(Index::V112),
    None,
    None,
    None,
    None,
    Some(Index::V113),
    Some(Index::V114),
    None,
    None,
    None,
    Some(Index::V115),
    None,
    Some(Index::V116),
    Some(Index::V117),
    Some(Index::V118),
    Some(Index::V119),
    None,
    Some(Index::V120),
    None,
    Some(Index::V121),
    Some(Index::V122),
    None,
    None,
    Some(Index::V123),
    None,
    None,
    Some(Index::V124),
    Some(Index::V125),
    Some(Index::V126),
    None,
    None,
    None,
    Some(Index::V127),
    None,
    Some(Index::V128),
    None,
    Some(Index::V129),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V130),
    None,
    Some(Index::V131),
    Some(Index::V132),
    Some(Index::V133),
    Some(Index::V134),
    Some(Index::V135),
    Some(Index::V136),
    Some(Index::V137),
    Some(Index::V138),
    Some(Index::V139),
    Some(Index::V140),
    None,
    None,
    None,
    None,
    Some(Index::V141),
    None,
    Some(Index::V142),
    None,
    Some(Index::V143),
    None,
    None,
    None,
    None,
    Some(Index::V144),
    None,
    None,
    Some(Index::V145),
    Some(Index::V146),
    Some(Index::V147),
    Some(Index::V148),
    None,
    Some(Index::V149),
    None,
    None,
    None,
    None,
    Some(Index::V150),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V151),
    None,
    None,
    None,
    Some(Index::V152),
    Some(Index::V153),
    None,
    None,
    Some(Index::V154),
    Some(Index::V155),
    None,
    None,
    None,
    Some(Index::V156),
    None,
    Some(Index::V157),
    Some(Index::V158),
    None,
    Some(Index::V159),
    None,
    None,
    Some(Index::V160),
    None,
    Some(Index::V161),
    Some(Index::V162),
    None,
    None,
    Some(Index::V163),
    Some(Index::V164),
    None,
    Some(Index::V165),
    None,
    None,
    None,
    None,
    Some(Index::V166),
    None,
    None,
    None,
    None,
    Some(Index::V167),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V168),
    None,
    Some(Index::V169),
    None,
    None,
    None,
    Some(Index::V170),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V171),
    None,
    Some(Index::V172),
    None,
    Some(Index::V173),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V174),
    None,
    None,
    Some(Index::V175),
    None,
    None,
    None,
    Some(Index::V176),
    None,
    Some(Index::V177),
    None,
    None,
    None,
    None,
    Some(Index::V178),
    Some(Index::V179),
    None,
    Some(Index::V180),
    None,
    Some(Index::V181),
    None,
    None,
    Some(Index::V182),
    None,
    None,
    Some(Index::V183),
    None,
    None,
    None,
    Some(Index::V184),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V185),
    None,
    None,
    Some(Index::V186),
    None,
    None,
    Some(Index::V187),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V188),
    None,
    None,
    Some(Index::V189),
    Some(Index::V190),
    None,
    None,
    Some(Index::V191),
    None,
    None,
    None,
    None,
    Some(Index::V192),
    None,
    Some(Index::V193),
    Some(Index::V194),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V195),
    Some(Index::V196),
    None,
    Some(Index::V197),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V198),
    Some(Index::V199),
    None,
    None,
    Some(Index::V200),
    None,
    None,
    None,
    Some(Index::V201),
    Some(Index::V202),
    None,
    Some(Index::V203),
    None,
    None,
    None,
    None,
    Some(Index::V204),
    None,
    Some(Index::V205),
    None,
    None,
    Some(Index::V206),
    Some(Index::V207),
    Some(Index::V208),
    None,
    Some(Index::V209),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V210),
    Some(Index::V211),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V212),
    Some(Index::V213),
    Some(Index::V214),
    Some(Index::V215),
    Some(Index::V216),
    None,
    None,
    None,
    None,
    Some(Index::V217),
    Some(Index::V218),
    None,
    None,
    None,
    None,
    Some(Index::V219),
    None,
    None,
    Some(Index::V220),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V221),
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V222),
    Some(Index::V223),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V224),
    None,
    Some(Index::V225),
    Some(Index::V226),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V227),
    None,
    None,
    None,
    Some(Index::V228),
    None,
    None,
    None,
    Some(Index::V229),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V230),
    None,
    None,
    None,
    None,
    Some(Index::V231),
    None,
    None,
    Some(Index::V232),
    None,
    None,
    None,
    Some(Index::V233),
    Some(Index::V234),
    None,
    Some(Index::V235),
    Some(Index::V236),
    None,
    Some(Index::V237),
    Some(Index::V238),
    Some(Index::V239),
    Some(Index::V240),
    Some(Index::V241),
    Some(Index::V242),
    Some(Index::V243),
    Some(Index::V244),
    Some(Index::V245),
    Some(Index::V246),
    Some(Index::V247),
    Some(Index::V248),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V249),
    Some(Index::V250),
    Some(Index::V251),
    Some(Index::V252),
    None,
    Some(Index::V253),
    None,
    None,
    Some(Index::V254),
    None,
    None,
    Some(Index::V255),
    None,
    None,
    None,
    None,
    Some(Index::V256),
    Some(Index::V257),
    Some(Index::V258),
    None,
    None,
    None,
    Some(Index::V259),
    None,
    Some(Index::V260),
    None,
    None,
    None,
    Some(Index::V261),
    None,
    None,
    None,
    None,
    Some(Index::V262),
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V263),
    None,
    None,
    Some(Index::V264),
    None,
    Some(Index::V265),
    None,
    None,
    Some(Index::V266),
    Some(Index::V267),
    Some(Index::V268),
    None,
    None,
    None,
    None,
    Some(Index::V269),
    Some(Index::V270),
    None,
    Some(Index::V271),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V272),
    None,
    None,
    None,
    Some(Index::V273),
    None,
    Some(Index::V274),
    Some(Index::V275),
    Some(Index::V276),
    Some(Index::V277),
    Some(Index::V278),
    Some(Index::V279),
    Some(Index::V280),
    Some(Index::V281),
    None,
    Some(Index::V282),
    None,
    None,
    None,
    Some(Index::V283),
    None,
    None,
    None,
    None,
    Some(Index::V284),
    None,
    None,
    None,
    Some(Index::V285),
    Some(Index::V286),
    None,
    Some(Index::V287),
    Some(Index::V288),
    None,
    None,
    None,
    Some(Index::V289),
    Some(Index::V290),
    Some(Index::V291),
    None,
    Some(Index::V292),
    None,
    Some(Index::V293),
    Some(Index::V294),
    None,
    Some(Index::V295),
    None,
    None,
    None,
    Some(Index::V296),
    Some(Index::V297),
    None,
    Some(Index::V298),
    None,
    Some(Index::V299),
    None,
    Some(Index::V300),
    Some(Index::V301),
    None,
    None,
    None,
    None,
    Some(Index::V302),
    None,
    None,
    Some(Index::V303),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V304),
    Some(Index::V305),
    None,
    None,
    None,
    None,
    Some(Index::V306),
    None,
    Some(Index::V307),
    None,
    None,
    Some(Index::V308),
    None,
    None,
    None,
    Some(Index::V309),
    None,
    Some(Index::V310),
    None,
    None,
    Some(Index::V311),
    Some(Index::V312),
    None,
    Some(Index::V313),
    None,
    None,
    Some(Index::V314),
    Some(Index::V315),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V316),
    None,
    None,
    Some(Index::V317),
    Some(Index::V318),
    Some(Index::V319),
    None,
    None,
    Some(Index::V320),
    None,
    None,
    None,
    Some(Index::V321),
    None,
    None,
    Some(Index::V322),
    None,
    None,
    Some(Index::V323),
    Some(Index::V324),
    None,
    None,
    None,
    Some(Index::V325),
    None,
    None,
    Some(Index::V326),
    None,
    Some(Index::V327),
    None,
    Some(Index::V328),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V329),
    None,
    Some(Index::V330),
    Some(Index::V331),
    None,
    Some(Index::V332),
    Some(Index::V333),
    Some(Index::V334),
    None,
    None,
    None,
    Some(Index::V335),
    Some(Index::V336),
    Some(Index::V337),
    Some(Index::V338),
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V339),
    Some(Index::V340),
    None,
    Some(Index::V341),
    None,
    None,
    None,
    None,
    Some(Index::V342),
    Some(Index::V343),
    None,
    None,
    None,
    None,
    Some(Index::V344),
    None,
    None,
    Some(Index::V345),
    Some(Index::V346),
    None,
    Some(Index::V347),
    None,
    None,
    Some(Index::V348),
    Some(Index::V349),
    None,
    None,
    None,
    Some(Index::V350),
    Some(Index::V351),
    None,
    None,
    None,
    Some(Index::V352),
    Some(Index::V353),
    None,
    Some(Index::V354),
    Some(Index::V355),
    Some(Index::V356),
    Some(Index::V357),
    Some(Index::V358),
    Some(Index::V359),
    Some(Index::V360),
    Some(Index::V361),
    Some(Index::V362),
    None,
    Some(Index::V363),
    Some(Index::V364),
    Some(Index::V365),
    Some(Index::V366),
    Some(Index::V367),
    Some(Index::V368),
    None,
    Some(Index::V369),
    Some(Index::V370),
    Some(Index::V371),
    None,
    Some(Index::V372),
    Some(Index::V373),
    Some(Index::V374),
    Some(Index::V375),
    Some(Index::V376),
    Some(Index::V377),
    Some(Index::V378),
    Some(Index::V379),
    Some(Index::V380),
    None,
    Some(Index::V381),
    Some(Index::V382),
    Some(Index::V383),
    Some(Index::V384),
    Some(Index::V385),
    None,
    None,
    Some(Index::V386),
    None,
    Some(Index::V387),
    Some(Index::V388),
    None,
    Some(Index::V389),
    Some(Index::V390),
    Some(Index::V391),
    Some(Index::V392),
    Some(Index::V393),
    Some(Index::V394),
    Some(Index::V395),
    Some(Index::V396),
    Some(Index::V397),
    Some(Index::V398),
    Some(Index::V399),
    Some(Index::V400),
    None,
    None,
    Some(Index::V401),
    Some(Index::V402),
    None,
    None,
    None,
    Some(Index::V403),
    Some(Index::V404),
    Some(Index::V405),
    None,
    Some(Index::V406),
    None,
    Some(Index::V407),
    Some(Index::V408),
    None,
    None,
    Some(Index::V409),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V410),
    None,
    Some(Index::V411),
    Some(Index::V412),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V413),
    None,
    None,
    Some(Index::V414),
    Some(Index::V415),
    None,
    Some(Index::V416),
    None,
    None,
    None,
    None,
    Some(Index::V417),
    None,
    None,
    None,
    Some(Index::V418),
    None,
    None,
    Some(Index::V419),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V420),
    None,
    None,
    None,
    Some(Index::V421),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V422),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V423),
    None,
    None,
    None,
    Some(Index::V424),
    None,
    None,
    Some(Index::V425),
    Some(Index::V426),
    None,
    None,
    Some(Index::V427),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V428),
    None,
    None,
    None,
    None,
    Some(Index::V429),
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V430),
    None,
    None,
    Some(Index::V431),
    Some(Index::V432),
    None,
    None,
    None,
    None,
    Some(Index::V433),
    None,
    Some(Index::V434),
    Some(Index::V435),
    None,
    None,
    None,
    Some(Index::V436),
    Some(Index::V437),
    Some(Index::V438),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V439),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V440),
    None,
    Some(Index::V441),
    None,
    None,
    None,
    None,
    Some(Index::V442),
    None,
    Some(Index::V443),
    None,
    Some(Index::V444),
    Some(Index::V445),
    None,
    None,
    None,
    Some(Index::V446),
    None,
    None,
    Some(Index::V447),
    Some(Index::V448),
    None,
    Some(Index::V449),
    None,
    None,
    Some(Index::V450),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V451),
    None,
    None,
    Some(Index::V452),
    None,
    None,
    Some(Index::V453),
    None,
    None,
    None,
    None,
    Some(Index::V454),
    None,
    None,
    None,
    None,
    Some(Index::V455),
    None,
    Some(Index::V456),
    None,
    None,
    Some(Index::V457),
    Some(Index::V458),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V459),
    Some(Index::V460),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V461),
    Some(Index::V462),
    None,
    None,
    Some(Index::V463),
    None,
    None,
    Some(Index::V464),
    None,
    None,
    None,
    Some(Index::V465),
    Some(Index::V466),
    None,
    None,
    Some(Index::V467),
    None,
    None,
    Some(Index::V468),
    None,
    Some(Index::V469),
    Some(Index::V470),
    Some(Index::V471),
    None,
    None,
    None,
    None,
    Some(Index::V472),
    None,
    Some(Index::V473),
    None,
    None,
    Some(Index::V474),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V475),
    None,
    Some(Index::V476),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V477),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V478),
    Some(Index::V479),
    None,
    None,
    None,
    None,
    Some(Index::V480),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V481),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V482),
    None,
    None,
    None,
    None,
    Some(Index::V483),
    None,
    None,
    Some(Index::V484),
    None,
    None,
    None,
    Some(Index::V485),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V486),
    Some(Index::V487),
    Some(Index::V488),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V489),
    None,
    None,
    None,
    None,
    Some(Index::V490),
    None,
    Some(Index::V491),
    None,
    None,
    None,
    Some(Index::V492),
    None,
    None,
    None,
    Some(Index::V493),
    None,
    None,
    None,
    Some(Index::V494),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V495),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V496),
    Some(Index::V497),
    None,
    None,
    Some(Index::V498),
    None,
    None,
    Some(Index::V499),
    Some(Index::V500),
    None,
    None,
    Some(Index::V501),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V502),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V503),
    None,
    None,
    Some(Index::V504),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V505),
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V506),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V507),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V508),
    Some(Index::V509),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V510),
    None,
    None,
    None,
    None,
    Some(Index::V511),
    Some(Index::V512),
    Some(Index::V513),
    None,
    None,
    Some(Index::V514),
    Some(Index::V515),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V516),
    None,
    None,
    None,
    Some(Index::V517),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V518),
    None,
    None,
    None,
    Some(Index::V519),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V520),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V521),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V522),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V523),
    None,
    Some(Index::V524),
    None,
    None,
    None,
    Some(Index::V525),
    None,
    None,
    Some(Index::V526),
    None,
    Some(Index::V527),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V528),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V529),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V530),
    None,
    Some(Index::V531),
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V532),
    None,
    Some(Index::V533),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V534),
    None,
    None,
    None,
    Some(Index::V535),
    None,
    Some(Index::V536),
    None,
    None,
    Some(Index::V537),
    Some(Index::V538),
    None,
    None,
    None,
    Some(Index::V539),
    Some(Index::V540),
    None,
    Some(Index::V541),
    Some(Index::V542),
    None,
    Some(Index::V543),
    None,
    Some(Index::V544),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V545),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V546),
    None,
    Some(Index::V547),
    None,
    None,
    Some(Index::V548),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V549),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V550),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V551),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V552),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V553),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V554),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V555),
    Some(Index::V556),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V557),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V558),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V559),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V560),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V561),
    None,
    None,
    None,
    None,
    None,
    Some(Index::V562),
    None,
    None,
    None,
    None,
    Some(Index::V563),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V564),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V565),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V566),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V567),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V568),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V569),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Index::V570),
];

struct Item(
    TimeZoneRef<'static>,
    #[cfg(feature = "binary")] &'static [u8],
);
const ITEMS: [(&[u8], Item); 571] = [
    (
        b"EST",
        Item(
            tzdata::EST,
            #[cfg(feature = "binary")]
            raw_tzdata::EST,
        ),
    ),
    (
        b"EST5EDT",
        Item(
            tzdata::EST_5_EDT,
            #[cfg(feature = "binary")]
            raw_tzdata::EST_5_EDT,
        ),
    ),
    (
        b"GMT",
        Item(
            tzdata::GMT,
            #[cfg(feature = "binary")]
            raw_tzdata::GMT,
        ),
    ),
    (
        b"GMT0",
        Item(
            tzdata::GMT,
            #[cfg(feature = "binary")]
            raw_tzdata::GMT,
        ),
    ),
    (
        b"GMT+0",
        Item(
            tzdata::GMT,
            #[cfg(feature = "binary")]
            raw_tzdata::GMT,
        ),
    ),
    (
        b"GMT-0",
        Item(
            tzdata::GMT,
            #[cfg(feature = "binary")]
            raw_tzdata::GMT,
        ),
    ),
    (
        b"Asia/Macao",
        Item(
            tzdata::ASIA_MACAO,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_MACAO,
        ),
    ),
    (
        b"Asia/Manila",
        Item(
            tzdata::ASIA_MANILA,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_MANILA,
        ),
    ),
    (
        b"Asia/Amman",
        Item(
            tzdata::ASIA_AMMAN,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_AMMAN,
        ),
    ),
    (
        b"America/Tortola",
        Item(
            tzdata::AMERICA_TORTOLA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_TORTOLA,
        ),
    ),
    (
        b"America/Noronha",
        Item(
            tzdata::AMERICA_NORONHA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_NORONHA,
        ),
    ),
    (
        b"Asia/Nicosia",
        Item(
            tzdata::ASIA_NICOSIA,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_NICOSIA,
        ),
    ),
    (
        b"America/Toronto",
        Item(
            tzdata::AMERICA_MONTREAL,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_MONTREAL,
        ),
    ),
    (
        b"America/Antigua",
        Item(
            tzdata::AMERICA_ANTIGUA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_ANTIGUA,
        ),
    ),
    (
        b"Indian/Mahe",
        Item(
            tzdata::INDIAN_MAHE,
            #[cfg(feature = "binary")]
            raw_tzdata::INDIAN_MAHE,
        ),
    ),
    (
        b"America/Ojinaga",
        Item(
            tzdata::AMERICA_OJINAGA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_OJINAGA,
        ),
    ),
    (
        b"America/Detroit",
        Item(
            tzdata::AMERICA_DETROIT,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_DETROIT,
        ),
    ),
    (
        b"Indian/Mayotte",
        Item(
            tzdata::INDIAN_MAYOTTE,
            #[cfg(feature = "binary")]
            raw_tzdata::INDIAN_MAYOTTE,
        ),
    ),
    (
        b"America/Araguaina",
        Item(
            tzdata::AMERICA_ARAGUAINA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_ARAGUAINA,
        ),
    ),
    (
        b"America/Guatemala",
        Item(
            tzdata::AMERICA_GUATEMALA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_GUATEMALA,
        ),
    ),
    (
        b"Iran",
        Item(
            tzdata::IRAN,
            #[cfg(feature = "binary")]
            raw_tzdata::IRAN,
        ),
    ),
    (
        b"America/Grenada",
        Item(
            tzdata::AMERICA_GRENADA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_GRENADA,
        ),
    ),
    (
        b"America/Blanc-Sablon",
        Item(
            tzdata::AMERICA_BLANC_SABLON,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_BLANC_SABLON,
        ),
    ),
    (
        b"America/Rainy_River",
        Item(
            tzdata::AMERICA_RAINY_RIVER,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_RAINY_RIVER,
        ),
    ),
    (
        b"America/Guadeloupe",
        Item(
            tzdata::AMERICA_GUADELOUPE,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_GUADELOUPE,
        ),
    ),
    (
        b"America/Regina",
        Item(
            tzdata::AMERICA_REGINA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_REGINA,
        ),
    ),
    (
        b"Indian/Comoro",
        Item(
            tzdata::INDIAN_COMORO,
            #[cfg(feature = "binary")]
            raw_tzdata::INDIAN_COMORO,
        ),
    ),
    (
        b"Indian/Antananarivo",
        Item(
            tzdata::INDIAN_ANTANANARIVO,
            #[cfg(feature = "binary")]
            raw_tzdata::INDIAN_ANTANANARIVO,
        ),
    ),
    (
        b"America/Bogota",
        Item(
            tzdata::AMERICA_BOGOTA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_BOGOTA,
        ),
    ),
    (
        b"Asia/Anadyr",
        Item(
            tzdata::ASIA_ANADYR,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_ANADYR,
        ),
    ),
    (
        b"Asia/Muscat",
        Item(
            tzdata::ASIA_MUSCAT,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_MUSCAT,
        ),
    ),
    (
        b"CST6CDT",
        Item(
            tzdata::CST_6_CDT,
            #[cfg(feature = "binary")]
            raw_tzdata::CST_6_CDT,
        ),
    ),
    (
        b"Africa/Monrovia",
        Item(
            tzdata::AFRICA_MONROVIA,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_MONROVIA,
        ),
    ),
    (
        b"Africa/Ceuta",
        Item(
            tzdata::AFRICA_CEUTA,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_CEUTA,
        ),
    ),
    (
        b"Africa/Dakar",
        Item(
            tzdata::AFRICA_DAKAR,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_DAKAR,
        ),
    ),
    (
        b"America/Santiago",
        Item(
            tzdata::AMERICA_SANTIAGO,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_SANTIAGO,
        ),
    ),
    (
        b"America/Godthab",
        Item(
            tzdata::AMERICA_GODTHAB,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_GODTHAB,
        ),
    ),
    (
        b"America/Ensenada",
        Item(
            tzdata::AMERICA_ENSENADA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_ENSENADA,
        ),
    ),
    (
        b"America/Santo_Domingo",
        Item(
            tzdata::AMERICA_SANTO_DOMINGO,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_SANTO_DOMINGO,
        ),
    ),
    (
        b"Asia/Dacca",
        Item(
            tzdata::ASIA_DACCA,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_DACCA,
        ),
    ),
    (
        b"Asia/Rangoon",
        Item(
            tzdata::ASIA_RANGOON,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_RANGOON,
        ),
    ),
    (
        b"America/Rosario",
        Item(
            tzdata::AMERICA_CORDOBA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_CORDOBA,
        ),
    ),
    (
        b"Asia/Dubai",
        Item(
            tzdata::ASIA_DUBAI,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_DUBAI,
        ),
    ),
    (
        b"Africa/Harare",
        Item(
            tzdata::AFRICA_HARARE,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_HARARE,
        ),
    ),
    (
        b"Africa/Cairo",
        Item(
            tzdata::EGYPT,
            #[cfg(feature = "binary")]
            raw_tzdata::EGYPT,
        ),
    ),
    (
        b"America/Resolute",
        Item(
            tzdata::AMERICA_RESOLUTE,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_RESOLUTE,
        ),
    ),
    (
        b"Asia/Ashgabat",
        Item(
            tzdata::ASIA_ASHGABAT,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_ASHGABAT,
        ),
    ),
    (
        b"Asia/Harbin",
        Item(
            tzdata::PRC,
            #[cfg(feature = "binary")]
            raw_tzdata::PRC,
        ),
    ),
    (
        b"Africa/Nairobi",
        Item(
            tzdata::AFRICA_NAIROBI,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_NAIROBI,
        ),
    ),
    (
        b"Africa/Malabo",
        Item(
            tzdata::AFRICA_MALABO,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_MALABO,
        ),
    ),
    (
        b"Asia/Hebron",
        Item(
            tzdata::ASIA_HEBRON,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_HEBRON,
        ),
    ),
    (
        b"NZ-CHAT",
        Item(
            tzdata::NZ_CHAT,
            #[cfg(feature = "binary")]
            raw_tzdata::NZ_CHAT,
        ),
    ),
    (
        b"GB",
        Item(
            tzdata::GB,
            #[cfg(feature = "binary")]
            raw_tzdata::GB,
        ),
    ),
    (
        b"Africa/Freetown",
        Item(
            tzdata::AFRICA_FREETOWN,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_FREETOWN,
        ),
    ),
    (
        b"America/Catamarca",
        Item(
            tzdata::AMERICA_CATAMARCA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_CATAMARCA,
        ),
    ),
    (
        b"America/Scoresbysund",
        Item(
            tzdata::AMERICA_SCORESBYSUND,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_SCORESBYSUND,
        ),
    ),
    (
        b"Africa/Casablanca",
        Item(
            tzdata::AFRICA_CASABLANCA,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_CASABLANCA,
        ),
    ),
    (
        b"America/Curacao",
        Item(
            tzdata::AMERICA_CURACAO,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_CURACAO,
        ),
    ),
    (
        b"America/Buenos_Aires",
        Item(
            tzdata::AMERICA_BUENOS_AIRES,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_BUENOS_AIRES,
        ),
    ),
    (
        b"Indian/Reunion",
        Item(
            tzdata::INDIAN_REUNION,
            #[cfg(feature = "binary")]
            raw_tzdata::INDIAN_REUNION,
        ),
    ),
    (
        b"America/Cuiaba",
        Item(
            tzdata::AMERICA_CUIABA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_CUIABA,
        ),
    ),
    (
        b"UCT",
        Item(
            tzdata::UCT,
            #[cfg(feature = "binary")]
            raw_tzdata::UCT,
        ),
    ),
    (
        b"Asia/Bahrain",
        Item(
            tzdata::ASIA_BAHRAIN,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_BAHRAIN,
        ),
    ),
    (
        b"Indian/Mauritius",
        Item(
            tzdata::INDIAN_MAURITIUS,
            #[cfg(feature = "binary")]
            raw_tzdata::INDIAN_MAURITIUS,
        ),
    ),
    (
        b"America/Coral_Harbour",
        Item(
            tzdata::AMERICA_ATIKOKAN,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_ATIKOKAN,
        ),
    ),
    (
        b"America/Cordoba",
        Item(
            tzdata::AMERICA_CORDOBA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_CORDOBA,
        ),
    ),
    (
        b"America/Boise",
        Item(
            tzdata::AMERICA_BOISE,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_BOISE,
        ),
    ),
    (
        b"Indian/Maldives",
        Item(
            tzdata::INDIAN_MALDIVES,
            #[cfg(feature = "binary")]
            raw_tzdata::INDIAN_MALDIVES,
        ),
    ),
    (
        b"Africa/Mbabane",
        Item(
            tzdata::AFRICA_MBABANE,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_MBABANE,
        ),
    ),
    (
        b"America/Indianapolis",
        Item(
            tzdata::AMERICA_FORT_WAYNE,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_FORT_WAYNE,
        ),
    ),
    (
        b"America/Recife",
        Item(
            tzdata::AMERICA_RECIFE,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_RECIFE,
        ),
    ),
    (
        b"Asia/Hovd",
        Item(
            tzdata::ASIA_HOVD,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_HOVD,
        ),
    ),
    (
        b"MST",
        Item(
            tzdata::MST,
            #[cfg(feature = "binary")]
            raw_tzdata::MST,
        ),
    ),
    (
        b"Africa/Porto-Novo",
        Item(
            tzdata::AFRICA_PORTO_NOVO,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_PORTO_NOVO,
        ),
    ),
    (
        b"Asia/Macau",
        Item(
            tzdata::ASIA_MACAO,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_MACAO,
        ),
    ),
    (
        b"Asia/Qatar",
        Item(
            tzdata::ASIA_QATAR,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_QATAR,
        ),
    ),
    (
        b"Eire",
        Item(
            tzdata::EIRE,
            #[cfg(feature = "binary")]
            raw_tzdata::EIRE,
        ),
    ),
    (
        b"Canada/Mountain",
        Item(
            tzdata::AMERICA_EDMONTON,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_EDMONTON,
        ),
    ),
    (
        b"America/Barbados",
        Item(
            tzdata::AMERICA_BARBADOS,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_BARBADOS,
        ),
    ),
    (
        b"America/Whitehorse",
        Item(
            tzdata::AMERICA_WHITEHORSE,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_WHITEHORSE,
        ),
    ),
    (
        b"America/Winnipeg",
        Item(
            tzdata::AMERICA_WINNIPEG,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_WINNIPEG,
        ),
    ),
    (
        b"Asia/Tehran",
        Item(
            tzdata::IRAN,
            #[cfg(feature = "binary")]
            raw_tzdata::IRAN,
        ),
    ),
    (
        b"GB-Eire",
        Item(
            tzdata::GB,
            #[cfg(feature = "binary")]
            raw_tzdata::GB,
        ),
    ),
    (
        b"MST7MDT",
        Item(
            tzdata::MST_7_MDT,
            #[cfg(feature = "binary")]
            raw_tzdata::MST_7_MDT,
        ),
    ),
    (
        b"America/Asuncion",
        Item(
            tzdata::AMERICA_ASUNCION,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_ASUNCION,
        ),
    ),
    (
        b"Japan",
        Item(
            tzdata::JAPAN,
            #[cfg(feature = "binary")]
            raw_tzdata::JAPAN,
        ),
    ),
    (
        b"Asia/Chita",
        Item(
            tzdata::ASIA_CHITA,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_CHITA,
        ),
    ),
    (
        b"Africa/Asmara",
        Item(
            tzdata::AFRICA_ASMARA,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_ASMARA,
        ),
    ),
    (
        b"Africa/Asmera",
        Item(
            tzdata::AFRICA_ASMARA,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_ASMARA,
        ),
    ),
    (
        b"America/Costa_Rica",
        Item(
            tzdata::AMERICA_COSTA_RICA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_COSTA_RICA,
        ),
    ),
    (
        b"America/Creston",
        Item(
            tzdata::AMERICA_CRESTON,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_CRESTON,
        ),
    ),
    (
        b"Asia/Aqtobe",
        Item(
            tzdata::ASIA_AQTOBE,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_AQTOBE,
        ),
    ),
    (
        b"America/Caracas",
        Item(
            tzdata::AMERICA_CARACAS,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_CARACAS,
        ),
    ),
    (
        b"America/Eirunepe",
        Item(
            tzdata::AMERICA_EIRUNEPE,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_EIRUNEPE,
        ),
    ),
    (
        b"Indian/Cocos",
        Item(
            tzdata::INDIAN_COCOS,
            #[cfg(feature = "binary")]
            raw_tzdata::INDIAN_COCOS,
        ),
    ),
    (
        b"Asia/Saigon",
        Item(
            tzdata::ASIA_HO_CHI_MINH,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_HO_CHI_MINH,
        ),
    ),
    (
        b"Jamaica",
        Item(
            tzdata::JAMAICA,
            #[cfg(feature = "binary")]
            raw_tzdata::JAMAICA,
        ),
    ),
    (
        b"America/Anguilla",
        Item(
            tzdata::AMERICA_ANGUILLA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_ANGUILLA,
        ),
    ),
    (
        b"America/Aruba",
        Item(
            tzdata::AMERICA_ARUBA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_ARUBA,
        ),
    ),
    (
        b"Africa/Accra",
        Item(
            tzdata::AFRICA_ACCRA,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_ACCRA,
        ),
    ),
    (
        b"America/Tegucigalpa",
        Item(
            tzdata::AMERICA_TEGUCIGALPA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_TEGUCIGALPA,
        ),
    ),
    (
        b"America/Cancun",
        Item(
            tzdata::AMERICA_CANCUN,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_CANCUN,
        ),
    ),
    (
        b"Africa/Addis_Ababa",
        Item(
            tzdata::AFRICA_ADDIS_ABABA,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_ADDIS_ABABA,
        ),
    ),
    (
        b"America/Managua",
        Item(
            tzdata::AMERICA_MANAGUA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_MANAGUA,
        ),
    ),
    (
        b"PST8PDT",
        Item(
            tzdata::PST_8_PDT,
            #[cfg(feature = "binary")]
            raw_tzdata::PST_8_PDT,
        ),
    ),
    (
        b"US/East-Indiana",
        Item(
            tzdata::AMERICA_FORT_WAYNE,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_FORT_WAYNE,
        ),
    ),
    (
        b"Asia/Dili",
        Item(
            tzdata::ASIA_DILI,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_DILI,
        ),
    ),
    (
        b"America/Montserrat",
        Item(
            tzdata::AMERICA_MONTSERRAT,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_MONTSERRAT,
        ),
    ),
    (
        b"America/Nome",
        Item(
            tzdata::AMERICA_NOME,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_NOME,
        ),
    ),
    (
        b"America/Montevideo",
        Item(
            tzdata::AMERICA_MONTEVIDEO,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_MONTEVIDEO,
        ),
    ),
    (
        b"America/Edmonton",
        Item(
            tzdata::AMERICA_EDMONTON,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_EDMONTON,
        ),
    ),
    (
        b"America/Menominee",
        Item(
            tzdata::AMERICA_MENOMINEE,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_MENOMINEE,
        ),
    ),
    (
        b"America/Dawson",
        Item(
            tzdata::AMERICA_DAWSON,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_DAWSON,
        ),
    ),
    (
        b"America/Mendoza",
        Item(
            tzdata::AMERICA_MENDOZA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_MENDOZA,
        ),
    ),
    (
        b"America/Martinique",
        Item(
            tzdata::AMERICA_MARTINIQUE,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_MARTINIQUE,
        ),
    ),
    (
        b"America/Danmarkshavn",
        Item(
            tzdata::AMERICA_DANMARKSHAVN,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_DANMARKSHAVN,
        ),
    ),
    (
        b"America/Chicago",
        Item(
            tzdata::AMERICA_CHICAGO,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_CHICAGO,
        ),
    ),
    (
        b"Navajo",
        Item(
            tzdata::NAVAJO,
            #[cfg(feature = "binary")]
            raw_tzdata::NAVAJO,
        ),
    ),
    (
        b"Africa/Maputo",
        Item(
            tzdata::AFRICA_MAPUTO,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_MAPUTO,
        ),
    ),
    (
        b"US/Samoa",
        Item(
            tzdata::PACIFIC_PAGO_PAGO,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_PAGO_PAGO,
        ),
    ),
    (
        b"America/Merida",
        Item(
            tzdata::AMERICA_MERIDA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_MERIDA,
        ),
    ),
    (
        b"America/Marigot",
        Item(
            tzdata::AMERICA_MARIGOT,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_MARIGOT,
        ),
    ),
    (
        b"America/Denver",
        Item(
            tzdata::NAVAJO,
            #[cfg(feature = "binary")]
            raw_tzdata::NAVAJO,
        ),
    ),
    (
        b"Africa/Maseru",
        Item(
            tzdata::AFRICA_MASERU,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_MASERU,
        ),
    ),
    (
        b"America/Panama",
        Item(
            tzdata::AMERICA_PANAMA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_PANAMA,
        ),
    ),
    (
        b"America/Dominica",
        Item(
            tzdata::AMERICA_DOMINICA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_DOMINICA,
        ),
    ),
    (
        b"Asia/Colombo",
        Item(
            tzdata::ASIA_COLOMBO,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_COLOMBO,
        ),
    ),
    (
        b"Asia/Famagusta",
        Item(
            tzdata::ASIA_FAMAGUSTA,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_FAMAGUSTA,
        ),
    ),
    (
        b"Africa/Mogadishu",
        Item(
            tzdata::AFRICA_MOGADISHU,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_MOGADISHU,
        ),
    ),
    (
        b"America/Virgin",
        Item(
            tzdata::AMERICA_ST_THOMAS,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_ST_THOMAS,
        ),
    ),
    (
        b"Cuba",
        Item(
            tzdata::CUBA,
            #[cfg(feature = "binary")]
            raw_tzdata::CUBA,
        ),
    ),
    (
        b"America/Port_of_Spain",
        Item(
            tzdata::AMERICA_MARIGOT,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_MARIGOT,
        ),
    ),
    (
        b"America/Paramaribo",
        Item(
            tzdata::AMERICA_PARAMARIBO,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_PARAMARIBO,
        ),
    ),
    (
        b"America/Porto_Acre",
        Item(
            tzdata::AMERICA_PORTO_ACRE,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_PORTO_ACRE,
        ),
    ),
    (
        b"America/Porto_Velho",
        Item(
            tzdata::AMERICA_PORTO_VELHO,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_PORTO_VELHO,
        ),
    ),
    (
        b"America/Nipigon",
        Item(
            tzdata::AMERICA_NIPIGON,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_NIPIGON,
        ),
    ),
    (
        b"Asia/Singapore",
        Item(
            tzdata::SINGAPORE,
            #[cfg(feature = "binary")]
            raw_tzdata::SINGAPORE,
        ),
    ),
    (
        b"America/Port-au-Prince",
        Item(
            tzdata::AMERICA_PORT_AU_PRINCE,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_PORT_AU_PRINCE,
        ),
    ),
    (
        b"America/Puerto_Rico",
        Item(
            tzdata::AMERICA_PUERTO_RICO,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_PUERTO_RICO,
        ),
    ),
    (
        b"Asia/Ulan_Bator",
        Item(
            tzdata::ASIA_ULAANBAATAR,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_ULAANBAATAR,
        ),
    ),
    (
        b"Asia/Ulaanbaatar",
        Item(
            tzdata::ASIA_ULAANBAATAR,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_ULAANBAATAR,
        ),
    ),
    (
        b"Africa/Lome",
        Item(
            tzdata::AFRICA_LOME,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_LOME,
        ),
    ),
    (
        b"EET",
        Item(
            tzdata::EET,
            #[cfg(feature = "binary")]
            raw_tzdata::EET,
        ),
    ),
    (
        b"Asia/Beirut",
        Item(
            tzdata::ASIA_BEIRUT,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_BEIRUT,
        ),
    ),
    (
        b"America/Manaus",
        Item(
            tzdata::AMERICA_MANAUS,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_MANAUS,
        ),
    ),
    (
        b"America/Santarem",
        Item(
            tzdata::AMERICA_SANTAREM,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_SANTAREM,
        ),
    ),
    (
        b"America/Matamoros",
        Item(
            tzdata::AMERICA_MATAMOROS,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_MATAMOROS,
        ),
    ),
    (
        b"US/Indiana-Starke",
        Item(
            tzdata::AMERICA_KNOX_IN,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_KNOX_IN,
        ),
    ),
    (
        b"Canada/Atlantic",
        Item(
            tzdata::AMERICA_HALIFAX,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_HALIFAX,
        ),
    ),
    (
        b"Africa/Ndjamena",
        Item(
            tzdata::AFRICA_NDJAMENA,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_NDJAMENA,
        ),
    ),
    (
        b"ROC",
        Item(
            tzdata::ROC,
            #[cfg(feature = "binary")]
            raw_tzdata::ROC,
        ),
    ),
    (
        b"Asia/Calcutta",
        Item(
            tzdata::ASIA_CALCUTTA,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_CALCUTTA,
        ),
    ),
    (
        b"Africa/Libreville",
        Item(
            tzdata::AFRICA_LIBREVILLE,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_LIBREVILLE,
        ),
    ),
    (
        b"America/Phoenix",
        Item(
            tzdata::AMERICA_PHOENIX,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_PHOENIX,
        ),
    ),
    (
        b"America/Pangnirtung",
        Item(
            tzdata::AMERICA_PANGNIRTUNG,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_PANGNIRTUNG,
        ),
    ),
    (
        b"America/Moncton",
        Item(
            tzdata::AMERICA_MONCTON,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_MONCTON,
        ),
    ),
    (
        b"America/Rio_Branco",
        Item(
            tzdata::AMERICA_PORTO_ACRE,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_PORTO_ACRE,
        ),
    ),
    (
        b"America/Nassau",
        Item(
            tzdata::AMERICA_NASSAU,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_NASSAU,
        ),
    ),
    (
        b"Africa/Djibouti",
        Item(
            tzdata::AFRICA_DJIBOUTI,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_DJIBOUTI,
        ),
    ),
    (
        b"America/Maceio",
        Item(
            tzdata::AMERICA_MACEIO,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_MACEIO,
        ),
    ),
    (
        b"Asia/Brunei",
        Item(
            tzdata::ASIA_BRUNEI,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_BRUNEI,
        ),
    ),
    (
        b"America/Punta_Arenas",
        Item(
            tzdata::AMERICA_PUNTA_ARENAS,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_PUNTA_ARENAS,
        ),
    ),
    (
        b"Asia/Damascus",
        Item(
            tzdata::ASIA_DAMASCUS,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_DAMASCUS,
        ),
    ),
    (
        b"America/Vancouver",
        Item(
            tzdata::AMERICA_VANCOUVER,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_VANCOUVER,
        ),
    ),
    (
        b"Asia/Aden",
        Item(
            tzdata::ASIA_ADEN,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_ADEN,
        ),
    ),
    (
        b"America/Boa_Vista",
        Item(
            tzdata::AMERICA_BOA_VISTA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_BOA_VISTA,
        ),
    ),
    (
        b"Asia/Aqtau",
        Item(
            tzdata::ASIA_AQTAU,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_AQTAU,
        ),
    ),
    (
        b"Africa/Abidjan",
        Item(
            tzdata::AFRICA_ABIDJAN,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_ABIDJAN,
        ),
    ),
    (
        b"America/Fortaleza",
        Item(
            tzdata::AMERICA_FORTALEZA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_FORTALEZA,
        ),
    ),
    (
        b"America/Fort_Nelson",
        Item(
            tzdata::AMERICA_FORT_NELSON,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_FORT_NELSON,
        ),
    ),
    (
        b"America/Fort_Wayne",
        Item(
            tzdata::AMERICA_FORT_WAYNE,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_FORT_WAYNE,
        ),
    ),
    (
        b"Asia/Chongqing",
        Item(
            tzdata::PRC,
            #[cfg(feature = "binary")]
            raw_tzdata::PRC,
        ),
    ),
    (
        b"PRC",
        Item(
            tzdata::PRC,
            #[cfg(feature = "binary")]
            raw_tzdata::PRC,
        ),
    ),
    (
        b"Africa/Sao_Tome",
        Item(
            tzdata::AFRICA_SAO_TOME,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_SAO_TOME,
        ),
    ),
    (
        b"America/Tijuana",
        Item(
            tzdata::AMERICA_ENSENADA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_ENSENADA,
        ),
    ),
    (
        b"America/Juneau",
        Item(
            tzdata::AMERICA_JUNEAU,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_JUNEAU,
        ),
    ),
    (
        b"Asia/Ust-Nera",
        Item(
            tzdata::ASIA_UST_NERA,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_UST_NERA,
        ),
    ),
    (
        b"CET",
        Item(
            tzdata::CET,
            #[cfg(feature = "binary")]
            raw_tzdata::CET,
        ),
    ),
    (
        b"UTC",
        Item(
            tzdata::UCT,
            #[cfg(feature = "binary")]
            raw_tzdata::UCT,
        ),
    ),
    (
        b"America/Sao_Paulo",
        Item(
            tzdata::AMERICA_SAO_PAULO,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_SAO_PAULO,
        ),
    ),
    (
        b"Africa/Bangui",
        Item(
            tzdata::AFRICA_BANGUI,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_BANGUI,
        ),
    ),
    (
        b"Asia/Samarkand",
        Item(
            tzdata::ASIA_SAMARKAND,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_SAMARKAND,
        ),
    ),
    (
        b"America/El_Salvador",
        Item(
            tzdata::AMERICA_EL_SALVADOR,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_EL_SALVADOR,
        ),
    ),
    (
        b"America/Atka",
        Item(
            tzdata::AMERICA_ADAK,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_ADAK,
        ),
    ),
    (
        b"Asia/Dhaka",
        Item(
            tzdata::ASIA_DACCA,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_DACCA,
        ),
    ),
    (
        b"WET",
        Item(
            tzdata::WET,
            #[cfg(feature = "binary")]
            raw_tzdata::WET,
        ),
    ),
    (
        b"America/Jamaica",
        Item(
            tzdata::JAMAICA,
            #[cfg(feature = "binary")]
            raw_tzdata::JAMAICA,
        ),
    ),
    (
        b"America/Rankin_Inlet",
        Item(
            tzdata::AMERICA_RANKIN_INLET,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_RANKIN_INLET,
        ),
    ),
    (
        b"America/Atikokan",
        Item(
            tzdata::AMERICA_ATIKOKAN,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_ATIKOKAN,
        ),
    ),
    (
        b"Singapore",
        Item(
            tzdata::SINGAPORE,
            #[cfg(feature = "binary")]
            raw_tzdata::SINGAPORE,
        ),
    ),
    (
        b"W-SU",
        Item(
            tzdata::W_SU,
            #[cfg(feature = "binary")]
            raw_tzdata::W_SU,
        ),
    ),
    (
        b"Africa/Lagos",
        Item(
            tzdata::AFRICA_LAGOS,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_LAGOS,
        ),
    ),
    (
        b"Australia/Tasmania",
        Item(
            tzdata::AUSTRALIA_CURRIE,
            #[cfg(feature = "binary")]
            raw_tzdata::AUSTRALIA_CURRIE,
        ),
    ),
    (
        b"Africa/Bissau",
        Item(
            tzdata::AFRICA_BISSAU,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_BISSAU,
        ),
    ),
    (
        b"America/Grand_Turk",
        Item(
            tzdata::AMERICA_GRAND_TURK,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_GRAND_TURK,
        ),
    ),
    (
        b"America/Swift_Current",
        Item(
            tzdata::AMERICA_SWIFT_CURRENT,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_SWIFT_CURRENT,
        ),
    ),
    (
        b"Asia/Qostanay",
        Item(
            tzdata::ASIA_QOSTANAY,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_QOSTANAY,
        ),
    ),
    (
        b"Australia/Darwin",
        Item(
            tzdata::AUSTRALIA_DARWIN,
            #[cfg(feature = "binary")]
            raw_tzdata::AUSTRALIA_DARWIN,
        ),
    ),
    (
        b"Asia/Pontianak",
        Item(
            tzdata::ASIA_PONTIANAK,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_PONTIANAK,
        ),
    ),
    (
        b"Australia/Adelaide",
        Item(
            tzdata::AUSTRALIA_ADELAIDE,
            #[cfg(feature = "binary")]
            raw_tzdata::AUSTRALIA_ADELAIDE,
        ),
    ),
    (
        b"America/St_Thomas",
        Item(
            tzdata::AMERICA_ST_THOMAS,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_ST_THOMAS,
        ),
    ),
    (
        b"America/Miquelon",
        Item(
            tzdata::AMERICA_MIQUELON,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_MIQUELON,
        ),
    ),
    (
        b"America/Knox_IN",
        Item(
            tzdata::AMERICA_KNOX_IN,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_KNOX_IN,
        ),
    ),
    (
        b"Africa/Luanda",
        Item(
            tzdata::AFRICA_LUANDA,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_LUANDA,
        ),
    ),
    (
        b"Asia/Hong_Kong",
        Item(
            tzdata::HONGKONG,
            #[cfg(feature = "binary")]
            raw_tzdata::HONGKONG,
        ),
    ),
    (
        b"Poland",
        Item(
            tzdata::POLAND,
            #[cfg(feature = "binary")]
            raw_tzdata::POLAND,
        ),
    ),
    (
        b"Asia/Qyzylorda",
        Item(
            tzdata::ASIA_QYZYLORDA,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_QYZYLORDA,
        ),
    ),
    (
        b"Asia/Vientiane",
        Item(
            tzdata::ASIA_VIENTIANE,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_VIENTIANE,
        ),
    ),
    (
        b"Africa/Nouakchott",
        Item(
            tzdata::AFRICA_NOUAKCHOTT,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_NOUAKCHOTT,
        ),
    ),
    (
        b"America/Sitka",
        Item(
            tzdata::AMERICA_SITKA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_SITKA,
        ),
    ),
    (
        b"Australia/Brisbane",
        Item(
            tzdata::AUSTRALIA_BRISBANE,
            #[cfg(feature = "binary")]
            raw_tzdata::AUSTRALIA_BRISBANE,
        ),
    ),
    (
        b"Asia/Ashkhabad",
        Item(
            tzdata::ASIA_ASHGABAT,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_ASHGABAT,
        ),
    ),
    (
        b"Pacific/Efate",
        Item(
            tzdata::PACIFIC_EFATE,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_EFATE,
        ),
    ),
    (
        b"Pacific/Tarawa",
        Item(
            tzdata::PACIFIC_TARAWA,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_TARAWA,
        ),
    ),
    (
        b"MET",
        Item(
            tzdata::MET,
            #[cfg(feature = "binary")]
            raw_tzdata::MET,
        ),
    ),
    (
        b"US/Eastern",
        Item(
            tzdata::AMERICA_NEW_YORK,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_NEW_YORK,
        ),
    ),
    (
        b"America/Guyana",
        Item(
            tzdata::AMERICA_GUYANA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_GUYANA,
        ),
    ),
    (
        b"Pacific/Apia",
        Item(
            tzdata::PACIFIC_APIA,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_APIA,
        ),
    ),
    (
        b"Canada/Eastern",
        Item(
            tzdata::AMERICA_MONTREAL,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_MONTREAL,
        ),
    ),
    (
        b"Africa/Johannesburg",
        Item(
            tzdata::AFRICA_JOHANNESBURG,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_JOHANNESBURG,
        ),
    ),
    (
        b"Pacific/Rarotonga",
        Item(
            tzdata::PACIFIC_RAROTONGA,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_RAROTONGA,
        ),
    ),
    (
        b"Iceland",
        Item(
            tzdata::ICELAND,
            #[cfg(feature = "binary")]
            raw_tzdata::ICELAND,
        ),
    ),
    (
        b"NZ",
        Item(
            tzdata::NZ,
            #[cfg(feature = "binary")]
            raw_tzdata::NZ,
        ),
    ),
    (
        b"Africa/Tripoli",
        Item(
            tzdata::LIBYA,
            #[cfg(feature = "binary")]
            raw_tzdata::LIBYA,
        ),
    ),
    (
        b"America/Iqaluit",
        Item(
            tzdata::AMERICA_IQALUIT,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_IQALUIT,
        ),
    ),
    (
        b"Canada/Newfoundland",
        Item(
            tzdata::AMERICA_ST_JOHNS,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_ST_JOHNS,
        ),
    ),
    (
        b"Asia/Kuwait",
        Item(
            tzdata::ASIA_KUWAIT,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_KUWAIT,
        ),
    ),
    (
        b"Australia/ACT",
        Item(
            tzdata::AUSTRALIA_ACT,
            #[cfg(feature = "binary")]
            raw_tzdata::AUSTRALIA_ACT,
        ),
    ),
    (
        b"Australia/Canberra",
        Item(
            tzdata::AUSTRALIA_ACT,
            #[cfg(feature = "binary")]
            raw_tzdata::AUSTRALIA_ACT,
        ),
    ),
    (
        b"Europe/Malta",
        Item(
            tzdata::EUROPE_MALTA,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_MALTA,
        ),
    ),
    (
        b"Europe/Samara",
        Item(
            tzdata::EUROPE_SAMARA,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_SAMARA,
        ),
    ),
    (
        b"Canada/Saskatchewan",
        Item(
            tzdata::AMERICA_REGINA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_REGINA,
        ),
    ),
    (
        b"US/Pacific",
        Item(
            tzdata::AMERICA_LOS_ANGELES,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_LOS_ANGELES,
        ),
    ),
    (
        b"Asia/Makassar",
        Item(
            tzdata::ASIA_MAKASSAR,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_MAKASSAR,
        ),
    ),
    (
        b"America/Belize",
        Item(
            tzdata::AMERICA_BELIZE,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_BELIZE,
        ),
    ),
    (
        b"Europe/Sofia",
        Item(
            tzdata::EUROPE_SOFIA,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_SOFIA,
        ),
    ),
    (
        b"Australia/West",
        Item(
            tzdata::AUSTRALIA_PERTH,
            #[cfg(feature = "binary")]
            raw_tzdata::AUSTRALIA_PERTH,
        ),
    ),
    (
        b"Europe/Mariehamn",
        Item(
            tzdata::EUROPE_HELSINKI,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_HELSINKI,
        ),
    ),
    (
        b"Pacific/Easter",
        Item(
            tzdata::CHILE_EASTER_ISLAND,
            #[cfg(feature = "binary")]
            raw_tzdata::CHILE_EASTER_ISLAND,
        ),
    ),
    (
        b"Africa/Bamako",
        Item(
            tzdata::AFRICA_BAMAKO,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_BAMAKO,
        ),
    ),
    (
        b"Europe/Copenhagen",
        Item(
            tzdata::EUROPE_COPENHAGEN,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_COPENHAGEN,
        ),
    ),
    (
        b"Asia/Yangon",
        Item(
            tzdata::ASIA_RANGOON,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_RANGOON,
        ),
    ),
    (
        b"Africa/Douala",
        Item(
            tzdata::AFRICA_DOUALA,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_DOUALA,
        ),
    ),
    (
        b"Asia/Novosibirsk",
        Item(
            tzdata::ASIA_NOVOSIBIRSK,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_NOVOSIBIRSK,
        ),
    ),
    (
        b"America/Goose_Bay",
        Item(
            tzdata::AMERICA_GOOSE_BAY,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_GOOSE_BAY,
        ),
    ),
    (
        b"America/Cayenne",
        Item(
            tzdata::AMERICA_CAYENNE,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_CAYENNE,
        ),
    ),
    (
        b"Africa/Tunis",
        Item(
            tzdata::AFRICA_TUNIS,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_TUNIS,
        ),
    ),
    (
        b"Europe/Zaporozhye",
        Item(
            tzdata::EUROPE_KIEV,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_KIEV,
        ),
    ),
    (
        b"America/Campo_Grande",
        Item(
            tzdata::AMERICA_CAMPO_GRANDE,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_CAMPO_GRANDE,
        ),
    ),
    (
        b"US/Arizona",
        Item(
            tzdata::AMERICA_PHOENIX,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_PHOENIX,
        ),
    ),
    (
        b"Asia/Kamchatka",
        Item(
            tzdata::ASIA_KAMCHATKA,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_KAMCHATKA,
        ),
    ),
    (
        b"America/Santa_Isabel",
        Item(
            tzdata::AMERICA_ENSENADA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_ENSENADA,
        ),
    ),
    (
        b"Europe/Madrid",
        Item(
            tzdata::EUROPE_MADRID,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_MADRID,
        ),
    ),
    (
        b"Africa/Dar_es_Salaam",
        Item(
            tzdata::AFRICA_DAR_ES_SALAAM,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_DAR_ES_SALAAM,
        ),
    ),
    (
        b"Europe/Andorra",
        Item(
            tzdata::EUROPE_ANDORRA,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_ANDORRA,
        ),
    ),
    (
        b"ROK",
        Item(
            tzdata::ROK,
            #[cfg(feature = "binary")]
            raw_tzdata::ROK,
        ),
    ),
    (
        b"America/St_Vincent",
        Item(
            tzdata::AMERICA_ST_VINCENT,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_ST_VINCENT,
        ),
    ),
    (
        b"America/Glace_Bay",
        Item(
            tzdata::AMERICA_GLACE_BAY,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_GLACE_BAY,
        ),
    ),
    (
        b"US/Michigan",
        Item(
            tzdata::AMERICA_DETROIT,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_DETROIT,
        ),
    ),
    (
        b"Europe/Zagreb",
        Item(
            tzdata::EUROPE_ZAGREB,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_ZAGREB,
        ),
    ),
    (
        b"Australia/Eucla",
        Item(
            tzdata::AUSTRALIA_EUCLA,
            #[cfg(feature = "binary")]
            raw_tzdata::AUSTRALIA_EUCLA,
        ),
    ),
    (
        b"Asia/Tomsk",
        Item(
            tzdata::ASIA_TOMSK,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_TOMSK,
        ),
    ),
    (
        b"HST",
        Item(
            tzdata::HST,
            #[cfg(feature = "binary")]
            raw_tzdata::HST,
        ),
    ),
    (
        b"Asia/Yerevan",
        Item(
            tzdata::ASIA_YEREVAN,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_YEREVAN,
        ),
    ),
    (
        b"Europe/Rome",
        Item(
            tzdata::EUROPE_ROME,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_ROME,
        ),
    ),
    (
        b"Africa/Juba",
        Item(
            tzdata::AFRICA_JUBA,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_JUBA,
        ),
    ),
    (
        b"Asia/Taipei",
        Item(
            tzdata::ROC,
            #[cfg(feature = "binary")]
            raw_tzdata::ROC,
        ),
    ),
    (
        b"Europe/Riga",
        Item(
            tzdata::EUROPE_RIGA,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_RIGA,
        ),
    ),
    (
        b"America/Dawson_Creek",
        Item(
            tzdata::AMERICA_DAWSON_CREEK,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_DAWSON_CREEK,
        ),
    ),
    (
        b"Australia/NSW",
        Item(
            tzdata::AUSTRALIA_ACT,
            #[cfg(feature = "binary")]
            raw_tzdata::AUSTRALIA_ACT,
        ),
    ),
    (
        b"Africa/Timbuktu",
        Item(
            tzdata::AFRICA_BAMAKO,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_BAMAKO,
        ),
    ),
    (
        b"America/Thunder_Bay",
        Item(
            tzdata::AMERICA_THUNDER_BAY,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_THUNDER_BAY,
        ),
    ),
    (
        b"Europe/Nicosia",
        Item(
            tzdata::ASIA_NICOSIA,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_NICOSIA,
        ),
    ),
    (
        b"Asia/Magadan",
        Item(
            tzdata::ASIA_MAGADAN,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_MAGADAN,
        ),
    ),
    (
        b"Asia/Gaza",
        Item(
            tzdata::ASIA_GAZA,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_GAZA,
        ),
    ),
    (
        b"America/St_Johns",
        Item(
            tzdata::AMERICA_ST_JOHNS,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_ST_JOHNS,
        ),
    ),
    (
        b"Australia/Melbourne",
        Item(
            tzdata::AUSTRALIA_MELBOURNE,
            #[cfg(feature = "binary")]
            raw_tzdata::AUSTRALIA_MELBOURNE,
        ),
    ),
    (
        b"Asia/Urumqi",
        Item(
            tzdata::ASIA_KASHGAR,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_KASHGAR,
        ),
    ),
    (
        b"Europe/Monaco",
        Item(
            tzdata::EUROPE_MONACO,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_MONACO,
        ),
    ),
    (
        b"Europe/Podgorica",
        Item(
            tzdata::EUROPE_BELGRADE,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_BELGRADE,
        ),
    ),
    (
        b"Europe/Athens",
        Item(
            tzdata::EUROPE_ATHENS,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_ATHENS,
        ),
    ),
    (
        b"Pacific/Niue",
        Item(
            tzdata::PACIFIC_NIUE,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_NIUE,
        ),
    ),
    (
        b"Asia/Chungking",
        Item(
            tzdata::PRC,
            #[cfg(feature = "binary")]
            raw_tzdata::PRC,
        ),
    ),
    (
        b"Europe/Astrakhan",
        Item(
            tzdata::EUROPE_ASTRAKHAN,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_ASTRAKHAN,
        ),
    ),
    (
        b"Australia/Victoria",
        Item(
            tzdata::AUSTRALIA_MELBOURNE,
            #[cfg(feature = "binary")]
            raw_tzdata::AUSTRALIA_MELBOURNE,
        ),
    ),
    (
        b"US/Alaska",
        Item(
            tzdata::AMERICA_ANCHORAGE,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_ANCHORAGE,
        ),
    ),
    (
        b"US/Aleutian",
        Item(
            tzdata::AMERICA_ADAK,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_ADAK,
        ),
    ),
    (
        b"America/Monterrey",
        Item(
            tzdata::AMERICA_MONTERREY,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_MONTERREY,
        ),
    ),
    (
        b"Africa/Blantyre",
        Item(
            tzdata::AFRICA_BLANTYRE,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_BLANTYRE,
        ),
    ),
    (
        b"Europe/Bratislava",
        Item(
            tzdata::EUROPE_BRATISLAVA,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_BRATISLAVA,
        ),
    ),
    (
        b"America/Thule",
        Item(
            tzdata::AMERICA_THULE,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_THULE,
        ),
    ),
    (
        b"Europe/Helsinki",
        Item(
            tzdata::EUROPE_HELSINKI,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_HELSINKI,
        ),
    ),
    (
        b"Israel",
        Item(
            tzdata::ISRAEL,
            #[cfg(feature = "binary")]
            raw_tzdata::ISRAEL,
        ),
    ),
    (
        b"Canada/Central",
        Item(
            tzdata::AMERICA_WINNIPEG,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_WINNIPEG,
        ),
    ),
    (
        b"Pacific/Tongatapu",
        Item(
            tzdata::PACIFIC_TONGATAPU,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_TONGATAPU,
        ),
    ),
    (
        b"Canada/Pacific",
        Item(
            tzdata::AMERICA_VANCOUVER,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_VANCOUVER,
        ),
    ),
    (
        b"Africa/Ouagadougou",
        Item(
            tzdata::AFRICA_OUAGADOUGOU,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_OUAGADOUGOU,
        ),
    ),
    (
        b"Europe/Vienna",
        Item(
            tzdata::EUROPE_VIENNA,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_VIENNA,
        ),
    ),
    (
        b"Australia/Queensland",
        Item(
            tzdata::AUSTRALIA_BRISBANE,
            #[cfg(feature = "binary")]
            raw_tzdata::AUSTRALIA_BRISBANE,
        ),
    ),
    (
        b"Asia/Dushanbe",
        Item(
            tzdata::ASIA_DUSHANBE,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_DUSHANBE,
        ),
    ),
    (
        b"Europe/Belgrade",
        Item(
            tzdata::EUROPE_BELGRADE,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_BELGRADE,
        ),
    ),
    (
        b"Africa/Algiers",
        Item(
            tzdata::AFRICA_ALGIERS,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_ALGIERS,
        ),
    ),
    (
        b"America/Louisville",
        Item(
            tzdata::AMERICA_LOUISVILLE,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_LOUISVILLE,
        ),
    ),
    (
        b"Asia/Shanghai",
        Item(
            tzdata::PRC,
            #[cfg(feature = "binary")]
            raw_tzdata::PRC,
        ),
    ),
    (
        b"Australia/Currie",
        Item(
            tzdata::AUSTRALIA_CURRIE,
            #[cfg(feature = "binary")]
            raw_tzdata::AUSTRALIA_CURRIE,
        ),
    ),
    (
        b"America/Lima",
        Item(
            tzdata::AMERICA_LIMA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_LIMA,
        ),
    ),
    (
        b"America/Montreal",
        Item(
            tzdata::AMERICA_MONTREAL,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_MONTREAL,
        ),
    ),
    (
        b"America/Metlakatla",
        Item(
            tzdata::AMERICA_METLAKATLA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_METLAKATLA,
        ),
    ),
    (
        b"America/Lower_Princes",
        Item(
            tzdata::AMERICA_CURACAO,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_CURACAO,
        ),
    ),
    (
        b"Africa/Bujumbura",
        Item(
            tzdata::AFRICA_BUJUMBURA,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_BUJUMBURA,
        ),
    ),
    (
        b"Pacific/Bougainville",
        Item(
            tzdata::PACIFIC_BOUGAINVILLE,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_BOUGAINVILLE,
        ),
    ),
    (
        b"America/Bahia",
        Item(
            tzdata::AMERICA_BAHIA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_BAHIA,
        ),
    ),
    (
        b"Pacific/Ponape",
        Item(
            tzdata::PACIFIC_POHNPEI,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_POHNPEI,
        ),
    ),
    (
        b"Europe/Volgograd",
        Item(
            tzdata::EUROPE_VOLGOGRAD,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_VOLGOGRAD,
        ),
    ),
    (
        b"Indian/Kerguelen",
        Item(
            tzdata::INDIAN_KERGUELEN,
            #[cfg(feature = "binary")]
            raw_tzdata::INDIAN_KERGUELEN,
        ),
    ),
    (
        b"Europe/Tirane",
        Item(
            tzdata::EUROPE_TIRANE,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_TIRANE,
        ),
    ),
    (
        b"US/Mountain",
        Item(
            tzdata::NAVAJO,
            #[cfg(feature = "binary")]
            raw_tzdata::NAVAJO,
        ),
    ),
    (
        b"America/Shiprock",
        Item(
            tzdata::NAVAJO,
            #[cfg(feature = "binary")]
            raw_tzdata::NAVAJO,
        ),
    ),
    (
        b"Europe/Sarajevo",
        Item(
            tzdata::EUROPE_SARAJEVO,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_SARAJEVO,
        ),
    ),
    (
        b"Chile/EasterIsland",
        Item(
            tzdata::CHILE_EASTER_ISLAND,
            #[cfg(feature = "binary")]
            raw_tzdata::CHILE_EASTER_ISLAND,
        ),
    ),
    (
        b"America/Mexico_City",
        Item(
            tzdata::AMERICA_MEXICO_CITY,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_MEXICO_CITY,
        ),
    ),
    (
        b"America/Belem",
        Item(
            tzdata::AMERICA_BELEM,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_BELEM,
        ),
    ),
    (
        b"Africa/Gaborone",
        Item(
            tzdata::AFRICA_GABORONE,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_GABORONE,
        ),
    ),
    (
        b"Africa/Niamey",
        Item(
            tzdata::AFRICA_NIAMEY,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_NIAMEY,
        ),
    ),
    (
        b"Europe/Saratov",
        Item(
            tzdata::EUROPE_SARATOV,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_SARATOV,
        ),
    ),
    (
        b"Asia/Choibalsan",
        Item(
            tzdata::ASIA_CHOIBALSAN,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_CHOIBALSAN,
        ),
    ),
    (
        b"Pacific/Gambier",
        Item(
            tzdata::PACIFIC_GAMBIER,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_GAMBIER,
        ),
    ),
    (
        b"Pacific/Samoa",
        Item(
            tzdata::PACIFIC_PAGO_PAGO,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_PAGO_PAGO,
        ),
    ),
    (
        b"Pacific/Pago_Pago",
        Item(
            tzdata::PACIFIC_PAGO_PAGO,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_PAGO_PAGO,
        ),
    ),
    (
        b"America/Cayman",
        Item(
            tzdata::AMERICA_CAYMAN,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_CAYMAN,
        ),
    ),
    (
        b"Europe/London",
        Item(
            tzdata::GB,
            #[cfg(feature = "binary")]
            raw_tzdata::GB,
        ),
    ),
    (
        b"Arctic/Longyearbyen",
        Item(
            tzdata::ARCTIC_LONGYEARBYEN,
            #[cfg(feature = "binary")]
            raw_tzdata::ARCTIC_LONGYEARBYEN,
        ),
    ),
    (
        b"America/Mazatlan",
        Item(
            tzdata::AMERICA_MAZATLAN,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_MAZATLAN,
        ),
    ),
    (
        b"Europe/Paris",
        Item(
            tzdata::EUROPE_PARIS,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_PARIS,
        ),
    ),
    (
        b"Asia/Vladivostok",
        Item(
            tzdata::ASIA_VLADIVOSTOK,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_VLADIVOSTOK,
        ),
    ),
    (
        b"Asia/Pyongyang",
        Item(
            tzdata::ASIA_PYONGYANG,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_PYONGYANG,
        ),
    ),
    (
        b"Europe/Moscow",
        Item(
            tzdata::W_SU,
            #[cfg(feature = "binary")]
            raw_tzdata::W_SU,
        ),
    ),
    (
        b"Europe/Vilnius",
        Item(
            tzdata::EUROPE_VILNIUS,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_VILNIUS,
        ),
    ),
    (
        b"America/Anchorage",
        Item(
            tzdata::AMERICA_ANCHORAGE,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_ANCHORAGE,
        ),
    ),
    (
        b"America/Adak",
        Item(
            tzdata::AMERICA_ADAK,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_ADAK,
        ),
    ),
    (
        b"Pacific/Chatham",
        Item(
            tzdata::NZ_CHAT,
            #[cfg(feature = "binary")]
            raw_tzdata::NZ_CHAT,
        ),
    ),
    (
        b"Europe/Vatican",
        Item(
            tzdata::EUROPE_ROME,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_ROME,
        ),
    ),
    (
        b"Europe/Warsaw",
        Item(
            tzdata::POLAND,
            #[cfg(feature = "binary")]
            raw_tzdata::POLAND,
        ),
    ),
    (
        b"America/Chihuahua",
        Item(
            tzdata::AMERICA_CHIHUAHUA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_CHIHUAHUA,
        ),
    ),
    (
        b"America/Inuvik",
        Item(
            tzdata::AMERICA_INUVIK,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_INUVIK,
        ),
    ),
    (
        b"Europe/Lisbon",
        Item(
            tzdata::PORTUGAL,
            #[cfg(feature = "binary")]
            raw_tzdata::PORTUGAL,
        ),
    ),
    (
        b"Africa/Kampala",
        Item(
            tzdata::AFRICA_KAMPALA,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_KAMPALA,
        ),
    ),
    (
        b"America/Bahia_Banderas",
        Item(
            tzdata::AMERICA_BAHIA_BANDERAS,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_BAHIA_BANDERAS,
        ),
    ),
    (
        b"Etc/GMT",
        Item(
            tzdata::GMT,
            #[cfg(feature = "binary")]
            raw_tzdata::GMT,
        ),
    ),
    (
        b"America/Cambridge_Bay",
        Item(
            tzdata::AMERICA_CAMBRIDGE_BAY,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_CAMBRIDGE_BAY,
        ),
    ),
    (
        b"Pacific/Saipan",
        Item(
            tzdata::PACIFIC_SAIPAN,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_SAIPAN,
        ),
    ),
    (
        b"US/Central",
        Item(
            tzdata::AMERICA_CHICAGO,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_CHICAGO,
        ),
    ),
    (
        b"Indian/Chagos",
        Item(
            tzdata::INDIAN_CHAGOS,
            #[cfg(feature = "binary")]
            raw_tzdata::INDIAN_CHAGOS,
        ),
    ),
    (
        b"Etc/GMT+0",
        Item(
            tzdata::GMT,
            #[cfg(feature = "binary")]
            raw_tzdata::GMT,
        ),
    ),
    (
        b"Asia/Tashkent",
        Item(
            tzdata::ASIA_TASHKENT,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_TASHKENT,
        ),
    ),
    (
        b"Europe/Prague",
        Item(
            tzdata::EUROPE_BRATISLAVA,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_BRATISLAVA,
        ),
    ),
    (
        b"Pacific/Nauru",
        Item(
            tzdata::PACIFIC_NAURU,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_NAURU,
        ),
    ),
    (
        b"Etc/GMT-0",
        Item(
            tzdata::GMT,
            #[cfg(feature = "binary")]
            raw_tzdata::GMT,
        ),
    ),
    (
        b"America/New_York",
        Item(
            tzdata::AMERICA_NEW_YORK,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_NEW_YORK,
        ),
    ),
    (
        b"Etc/UCT",
        Item(
            tzdata::UCT,
            #[cfg(feature = "binary")]
            raw_tzdata::UCT,
        ),
    ),
    (
        b"Pacific/Pitcairn",
        Item(
            tzdata::PACIFIC_PITCAIRN,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_PITCAIRN,
        ),
    ),
    (
        b"Etc/GMT0",
        Item(
            tzdata::GMT,
            #[cfg(feature = "binary")]
            raw_tzdata::GMT,
        ),
    ),
    (
        b"Etc/GMT+7",
        Item(
            tzdata::ETC_GMT_PLUS_7,
            #[cfg(feature = "binary")]
            raw_tzdata::ETC_GMT_PLUS_7,
        ),
    ),
    (
        b"Pacific/Funafuti",
        Item(
            tzdata::PACIFIC_FUNAFUTI,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_FUNAFUTI,
        ),
    ),
    (
        b"Europe/Busingen",
        Item(
            tzdata::EUROPE_BUSINGEN,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_BUSINGEN,
        ),
    ),
    (
        b"Etc/GMT+5",
        Item(
            tzdata::ETC_GMT_PLUS_5,
            #[cfg(feature = "binary")]
            raw_tzdata::ETC_GMT_PLUS_5,
        ),
    ),
    (
        b"Etc/GMT-7",
        Item(
            tzdata::ETC_GMT_MINUS_7,
            #[cfg(feature = "binary")]
            raw_tzdata::ETC_GMT_MINUS_7,
        ),
    ),
    (
        b"Etc/GMT+10",
        Item(
            tzdata::ETC_GMT_PLUS_10,
            #[cfg(feature = "binary")]
            raw_tzdata::ETC_GMT_PLUS_10,
        ),
    ),
    (
        b"Africa/Lusaka",
        Item(
            tzdata::AFRICA_LUSAKA,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_LUSAKA,
        ),
    ),
    (
        b"Etc/GMT-5",
        Item(
            tzdata::ETC_GMT_MINUS_5,
            #[cfg(feature = "binary")]
            raw_tzdata::ETC_GMT_MINUS_5,
        ),
    ),
    (
        b"Etc/GMT+8",
        Item(
            tzdata::ETC_GMT_PLUS_8,
            #[cfg(feature = "binary")]
            raw_tzdata::ETC_GMT_PLUS_8,
        ),
    ),
    (
        b"Etc/GMT-10",
        Item(
            tzdata::ETC_GMT_MINUS_10,
            #[cfg(feature = "binary")]
            raw_tzdata::ETC_GMT_MINUS_10,
        ),
    ),
    (
        b"Pacific/Marquesas",
        Item(
            tzdata::PACIFIC_MARQUESAS,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_MARQUESAS,
        ),
    ),
    (
        b"Etc/GMT+2",
        Item(
            tzdata::ETC_GMT_PLUS_2,
            #[cfg(feature = "binary")]
            raw_tzdata::ETC_GMT_PLUS_2,
        ),
    ),
    (
        b"Etc/GMT-8",
        Item(
            tzdata::ETC_GMT_MINUS_8,
            #[cfg(feature = "binary")]
            raw_tzdata::ETC_GMT_MINUS_8,
        ),
    ),
    (
        b"America/St_Kitts",
        Item(
            tzdata::AMERICA_ST_KITTS,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_ST_KITTS,
        ),
    ),
    (
        b"Africa/Lubumbashi",
        Item(
            tzdata::AFRICA_LUBUMBASHI,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_LUBUMBASHI,
        ),
    ),
    (
        b"Etc/GMT+12",
        Item(
            tzdata::ETC_GMT_PLUS_12,
            #[cfg(feature = "binary")]
            raw_tzdata::ETC_GMT_PLUS_12,
        ),
    ),
    (
        b"Etc/GMT-2",
        Item(
            tzdata::ETC_GMT_MINUS_2,
            #[cfg(feature = "binary")]
            raw_tzdata::ETC_GMT_MINUS_2,
        ),
    ),
    (
        b"Etc/GMT+1",
        Item(
            tzdata::ETC_GMT_PLUS_1,
            #[cfg(feature = "binary")]
            raw_tzdata::ETC_GMT_PLUS_1,
        ),
    ),
    (
        b"Etc/GMT+11",
        Item(
            tzdata::ETC_GMT_PLUS_11,
            #[cfg(feature = "binary")]
            raw_tzdata::ETC_GMT_PLUS_11,
        ),
    ),
    (
        b"Etc/GMT-12",
        Item(
            tzdata::ETC_GMT_MINUS_12,
            #[cfg(feature = "binary")]
            raw_tzdata::ETC_GMT_MINUS_12,
        ),
    ),
    (
        b"Factory",
        Item(
            tzdata::FACTORY,
            #[cfg(feature = "binary")]
            raw_tzdata::FACTORY,
        ),
    ),
    (
        b"Etc/GMT-1",
        Item(
            tzdata::ETC_GMT_MINUS_1,
            #[cfg(feature = "binary")]
            raw_tzdata::ETC_GMT_MINUS_1,
        ),
    ),
    (
        b"Etc/GMT-11",
        Item(
            tzdata::ETC_GMT_MINUS_11,
            #[cfg(feature = "binary")]
            raw_tzdata::ETC_GMT_MINUS_11,
        ),
    ),
    (
        b"Pacific/Noumea",
        Item(
            tzdata::PACIFIC_NOUMEA,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_NOUMEA,
        ),
    ),
    (
        b"Etc/GMT+6",
        Item(
            tzdata::ETC_GMT_PLUS_6,
            #[cfg(feature = "binary")]
            raw_tzdata::ETC_GMT_PLUS_6,
        ),
    ),
    (
        b"Etc/GMT+9",
        Item(
            tzdata::ETC_GMT_PLUS_9,
            #[cfg(feature = "binary")]
            raw_tzdata::ETC_GMT_PLUS_9,
        ),
    ),
    (
        b"Europe/Oslo",
        Item(
            tzdata::ARCTIC_LONGYEARBYEN,
            #[cfg(feature = "binary")]
            raw_tzdata::ARCTIC_LONGYEARBYEN,
        ),
    ),
    (
        b"Etc/GMT-6",
        Item(
            tzdata::ETC_GMT_MINUS_6,
            #[cfg(feature = "binary")]
            raw_tzdata::ETC_GMT_MINUS_6,
        ),
    ),
    (
        b"Etc/GMT-14",
        Item(
            tzdata::ETC_GMT_MINUS_14,
            #[cfg(feature = "binary")]
            raw_tzdata::ETC_GMT_MINUS_14,
        ),
    ),
    (
        b"Etc/GMT-9",
        Item(
            tzdata::ETC_GMT_MINUS_9,
            #[cfg(feature = "binary")]
            raw_tzdata::ETC_GMT_MINUS_9,
        ),
    ),
    (
        b"Etc/GMT-13",
        Item(
            tzdata::ETC_GMT_MINUS_13,
            #[cfg(feature = "binary")]
            raw_tzdata::ETC_GMT_MINUS_13,
        ),
    ),
    (
        b"Egypt",
        Item(
            tzdata::EGYPT,
            #[cfg(feature = "binary")]
            raw_tzdata::EGYPT,
        ),
    ),
    (
        b"Etc/GMT+4",
        Item(
            tzdata::ETC_GMT_PLUS_4,
            #[cfg(feature = "binary")]
            raw_tzdata::ETC_GMT_PLUS_4,
        ),
    ),
    (
        b"Asia/Jakarta",
        Item(
            tzdata::ASIA_JAKARTA,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_JAKARTA,
        ),
    ),
    (
        b"America/St_Barthelemy",
        Item(
            tzdata::AMERICA_MARIGOT,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_MARIGOT,
        ),
    ),
    (
        b"Pacific/Kanton",
        Item(
            tzdata::PACIFIC_ENDERBURY,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_ENDERBURY,
        ),
    ),
    (
        b"Etc/GMT+3",
        Item(
            tzdata::ETC_GMT_PLUS_3,
            #[cfg(feature = "binary")]
            raw_tzdata::ETC_GMT_PLUS_3,
        ),
    ),
    (
        b"Etc/GMT-4",
        Item(
            tzdata::ETC_GMT_MINUS_4,
            #[cfg(feature = "binary")]
            raw_tzdata::ETC_GMT_MINUS_4,
        ),
    ),
    (
        b"America/St_Lucia",
        Item(
            tzdata::AMERICA_ST_LUCIA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_ST_LUCIA,
        ),
    ),
    (
        b"Etc/GMT-3",
        Item(
            tzdata::ETC_GMT_MINUS_3,
            #[cfg(feature = "binary")]
            raw_tzdata::ETC_GMT_MINUS_3,
        ),
    ),
    (
        b"Europe/Brussels",
        Item(
            tzdata::EUROPE_BRUSSELS,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_BRUSSELS,
        ),
    ),
    (
        b"Asia/Seoul",
        Item(
            tzdata::ROK,
            #[cfg(feature = "binary")]
            raw_tzdata::ROK,
        ),
    ),
    (
        b"Indian/Christmas",
        Item(
            tzdata::INDIAN_CHRISTMAS,
            #[cfg(feature = "binary")]
            raw_tzdata::INDIAN_CHRISTMAS,
        ),
    ),
    (
        b"Brazil/DeNoronha",
        Item(
            tzdata::AMERICA_NORONHA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_NORONHA,
        ),
    ),
    (
        b"Australia/Yancowinna",
        Item(
            tzdata::AUSTRALIA_BROKEN_HILL,
            #[cfg(feature = "binary")]
            raw_tzdata::AUSTRALIA_BROKEN_HILL,
        ),
    ),
    (
        b"Canada/Yukon",
        Item(
            tzdata::AMERICA_WHITEHORSE,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_WHITEHORSE,
        ),
    ),
    (
        b"America/Havana",
        Item(
            tzdata::CUBA,
            #[cfg(feature = "binary")]
            raw_tzdata::CUBA,
        ),
    ),
    (
        b"Asia/Thimbu",
        Item(
            tzdata::ASIA_THIMBU,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_THIMBU,
        ),
    ),
    (
        b"Africa/El_Aaiun",
        Item(
            tzdata::AFRICA_EL_AAIUN,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_EL_AAIUN,
        ),
    ),
    (
        b"America/Hermosillo",
        Item(
            tzdata::AMERICA_HERMOSILLO,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_HERMOSILLO,
        ),
    ),
    (
        b"Mexico/BajaNorte",
        Item(
            tzdata::AMERICA_ENSENADA,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_ENSENADA,
        ),
    ),
    (
        b"Asia/Baku",
        Item(
            tzdata::ASIA_BAKU,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_BAKU,
        ),
    ),
    (
        b"Asia/Bangkok",
        Item(
            tzdata::ASIA_BANGKOK,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_BANGKOK,
        ),
    ),
    (
        b"Brazil/West",
        Item(
            tzdata::AMERICA_MANAUS,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_MANAUS,
        ),
    ),
    (
        b"Asia/Almaty",
        Item(
            tzdata::ASIA_ALMATY,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_ALMATY,
        ),
    ),
    (
        b"Asia/Atyrau",
        Item(
            tzdata::ASIA_ATYRAU,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_ATYRAU,
        ),
    ),
    (
        b"Pacific/Kiritimati",
        Item(
            tzdata::PACIFIC_KIRITIMATI,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_KIRITIMATI,
        ),
    ),
    (
        b"Pacific/Guam",
        Item(
            tzdata::PACIFIC_GUAM,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_GUAM,
        ),
    ),
    (
        b"Africa/Conakry",
        Item(
            tzdata::AFRICA_CONAKRY,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_CONAKRY,
        ),
    ),
    (
        b"Australia/Lord_Howe",
        Item(
            tzdata::AUSTRALIA_LHI,
            #[cfg(feature = "binary")]
            raw_tzdata::AUSTRALIA_LHI,
        ),
    ),
    (
        b"Australia/Lindeman",
        Item(
            tzdata::AUSTRALIA_LINDEMAN,
            #[cfg(feature = "binary")]
            raw_tzdata::AUSTRALIA_LINDEMAN,
        ),
    ),
    (
        b"America/Los_Angeles",
        Item(
            tzdata::AMERICA_LOS_ANGELES,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_LOS_ANGELES,
        ),
    ),
    (
        b"Asia/Tbilisi",
        Item(
            tzdata::ASIA_TBILISI,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_TBILISI,
        ),
    ),
    (
        b"Pacific/Enderbury",
        Item(
            tzdata::PACIFIC_ENDERBURY,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_ENDERBURY,
        ),
    ),
    (
        b"Pacific/Kosrae",
        Item(
            tzdata::PACIFIC_KOSRAE,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_KOSRAE,
        ),
    ),
    (
        b"America/Nuuk",
        Item(
            tzdata::AMERICA_GODTHAB,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_GODTHAB,
        ),
    ),
    (
        b"Australia/Broken_Hill",
        Item(
            tzdata::AUSTRALIA_BROKEN_HILL,
            #[cfg(feature = "binary")]
            raw_tzdata::AUSTRALIA_BROKEN_HILL,
        ),
    ),
    (
        b"Mexico/BajaSur",
        Item(
            tzdata::AMERICA_MAZATLAN,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_MAZATLAN,
        ),
    ),
    (
        b"Europe/Belfast",
        Item(
            tzdata::GB,
            #[cfg(feature = "binary")]
            raw_tzdata::GB,
        ),
    ),
    (
        b"Asia/Ujung_Pandang",
        Item(
            tzdata::ASIA_MAKASSAR,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_MAKASSAR,
        ),
    ),
    (
        b"America/Guayaquil",
        Item(
            tzdata::AMERICA_GUAYAQUIL,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_GUAYAQUIL,
        ),
    ),
    (
        b"Pacific/Auckland",
        Item(
            tzdata::NZ,
            #[cfg(feature = "binary")]
            raw_tzdata::NZ,
        ),
    ),
    (
        b"Asia/Khandyga",
        Item(
            tzdata::ASIA_KHANDYGA,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_KHANDYGA,
        ),
    ),
    (
        b"Antarctica/Rothera",
        Item(
            tzdata::ANTARCTICA_ROTHERA,
            #[cfg(feature = "binary")]
            raw_tzdata::ANTARCTICA_ROTHERA,
        ),
    ),
    (
        b"Brazil/Acre",
        Item(
            tzdata::AMERICA_PORTO_ACRE,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_PORTO_ACRE,
        ),
    ),
    (
        b"Europe/Amsterdam",
        Item(
            tzdata::EUROPE_AMSTERDAM,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_AMSTERDAM,
        ),
    ),
    (
        b"Asia/Omsk",
        Item(
            tzdata::ASIA_OMSK,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_OMSK,
        ),
    ),
    (
        b"Pacific/Wake",
        Item(
            tzdata::PACIFIC_WAKE,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_WAKE,
        ),
    ),
    (
        b"Antarctica/DumontDUrville",
        Item(
            tzdata::ANTARCTICA_DUMONT_D_URVILLE,
            #[cfg(feature = "binary")]
            raw_tzdata::ANTARCTICA_DUMONT_D_URVILLE,
        ),
    ),
    (
        b"Kwajalein",
        Item(
            tzdata::KWAJALEIN,
            #[cfg(feature = "binary")]
            raw_tzdata::KWAJALEIN,
        ),
    ),
    (
        b"America/Jujuy",
        Item(
            tzdata::AMERICA_JUJUY,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_JUJUY,
        ),
    ),
    (
        b"Atlantic/Jan_Mayen",
        Item(
            tzdata::ARCTIC_LONGYEARBYEN,
            #[cfg(feature = "binary")]
            raw_tzdata::ARCTIC_LONGYEARBYEN,
        ),
    ),
    (
        b"America/Yakutat",
        Item(
            tzdata::AMERICA_YAKUTAT,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_YAKUTAT,
        ),
    ),
    (
        b"Pacific/Guadalcanal",
        Item(
            tzdata::PACIFIC_GUADALCANAL,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_GUADALCANAL,
        ),
    ),
    (
        b"Atlantic/Faeroe",
        Item(
            tzdata::ATLANTIC_FAEROE,
            #[cfg(feature = "binary")]
            raw_tzdata::ATLANTIC_FAEROE,
        ),
    ),
    (
        b"Africa/Banjul",
        Item(
            tzdata::AFRICA_BANJUL,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_BANJUL,
        ),
    ),
    (
        b"Atlantic/Faroe",
        Item(
            tzdata::ATLANTIC_FAEROE,
            #[cfg(feature = "binary")]
            raw_tzdata::ATLANTIC_FAEROE,
        ),
    ),
    (
        b"Atlantic/Bermuda",
        Item(
            tzdata::ATLANTIC_BERMUDA,
            #[cfg(feature = "binary")]
            raw_tzdata::ATLANTIC_BERMUDA,
        ),
    ),
    (
        b"Atlantic/Madeira",
        Item(
            tzdata::ATLANTIC_MADEIRA,
            #[cfg(feature = "binary")]
            raw_tzdata::ATLANTIC_MADEIRA,
        ),
    ),
    (
        b"Asia/Katmandu",
        Item(
            tzdata::ASIA_KATHMANDU,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_KATHMANDU,
        ),
    ),
    (
        b"Pacific/Fiji",
        Item(
            tzdata::PACIFIC_FIJI,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_FIJI,
        ),
    ),
    (
        b"Asia/Istanbul",
        Item(
            tzdata::TURKEY,
            #[cfg(feature = "binary")]
            raw_tzdata::TURKEY,
        ),
    ),
    (
        b"Europe/San_Marino",
        Item(
            tzdata::EUROPE_ROME,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_ROME,
        ),
    ),
    (
        b"Pacific/Majuro",
        Item(
            tzdata::PACIFIC_MAJURO,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_MAJURO,
        ),
    ),
    (
        b"Antarctica/Syowa",
        Item(
            tzdata::ANTARCTICA_SYOWA,
            #[cfg(feature = "binary")]
            raw_tzdata::ANTARCTICA_SYOWA,
        ),
    ),
    (
        b"Europe/Budapest",
        Item(
            tzdata::EUROPE_BUDAPEST,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_BUDAPEST,
        ),
    ),
    (
        b"Zulu",
        Item(
            tzdata::UCT,
            #[cfg(feature = "binary")]
            raw_tzdata::UCT,
        ),
    ),
    (
        b"Antarctica/South_Pole",
        Item(
            tzdata::ANTARCTICA_MC_MURDO,
            #[cfg(feature = "binary")]
            raw_tzdata::ANTARCTICA_MC_MURDO,
        ),
    ),
    (
        b"Pacific/Kwajalein",
        Item(
            tzdata::KWAJALEIN,
            #[cfg(feature = "binary")]
            raw_tzdata::KWAJALEIN,
        ),
    ),
    (
        b"Europe/Kaliningrad",
        Item(
            tzdata::EUROPE_KALININGRAD,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_KALININGRAD,
        ),
    ),
    (
        b"Antarctica/Davis",
        Item(
            tzdata::ANTARCTICA_DAVIS,
            #[cfg(feature = "binary")]
            raw_tzdata::ANTARCTICA_DAVIS,
        ),
    ),
    (
        b"Asia/Kolkata",
        Item(
            tzdata::ASIA_CALCUTTA,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_CALCUTTA,
        ),
    ),
    (
        b"Pacific/Galapagos",
        Item(
            tzdata::PACIFIC_GALAPAGOS,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_GALAPAGOS,
        ),
    ),
    (
        b"Asia/Yekaterinburg",
        Item(
            tzdata::ASIA_YEKATERINBURG,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_YEKATERINBURG,
        ),
    ),
    (
        b"Etc/UTC",
        Item(
            tzdata::UCT,
            #[cfg(feature = "binary")]
            raw_tzdata::UCT,
        ),
    ),
    (
        b"Asia/Ho_Chi_Minh",
        Item(
            tzdata::ASIA_HO_CHI_MINH,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_HO_CHI_MINH,
        ),
    ),
    (
        b"Asia/Tokyo",
        Item(
            tzdata::JAPAN,
            #[cfg(feature = "binary")]
            raw_tzdata::JAPAN,
        ),
    ),
    (
        b"Libya",
        Item(
            tzdata::LIBYA,
            #[cfg(feature = "binary")]
            raw_tzdata::LIBYA,
        ),
    ),
    (
        b"US/Hawaii",
        Item(
            tzdata::PACIFIC_HONOLULU,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_HONOLULU,
        ),
    ),
    (
        b"Africa/Kigali",
        Item(
            tzdata::AFRICA_KIGALI,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_KIGALI,
        ),
    ),
    (
        b"Australia/North",
        Item(
            tzdata::AUSTRALIA_DARWIN,
            #[cfg(feature = "binary")]
            raw_tzdata::AUSTRALIA_DARWIN,
        ),
    ),
    (
        b"Australia/Hobart",
        Item(
            tzdata::AUSTRALIA_CURRIE,
            #[cfg(feature = "binary")]
            raw_tzdata::AUSTRALIA_CURRIE,
        ),
    ),
    (
        b"Asia/Tel_Aviv",
        Item(
            tzdata::ISRAEL,
            #[cfg(feature = "binary")]
            raw_tzdata::ISRAEL,
        ),
    ),
    (
        b"Asia/Jayapura",
        Item(
            tzdata::ASIA_JAYAPURA,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_JAYAPURA,
        ),
    ),
    (
        b"Europe/Isle_of_Man",
        Item(
            tzdata::EUROPE_ISLE_OF_MAN,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_ISLE_OF_MAN,
        ),
    ),
    (
        b"Asia/Srednekolymsk",
        Item(
            tzdata::ASIA_SREDNEKOLYMSK,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_SREDNEKOLYMSK,
        ),
    ),
    (
        b"Asia/Kashgar",
        Item(
            tzdata::ASIA_KASHGAR,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_KASHGAR,
        ),
    ),
    (
        b"Europe/Berlin",
        Item(
            tzdata::EUROPE_BERLIN,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_BERLIN,
        ),
    ),
    (
        b"Europe/Luxembourg",
        Item(
            tzdata::EUROPE_LUXEMBOURG,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_LUXEMBOURG,
        ),
    ),
    (
        b"Australia/South",
        Item(
            tzdata::AUSTRALIA_ADELAIDE,
            #[cfg(feature = "binary")]
            raw_tzdata::AUSTRALIA_ADELAIDE,
        ),
    ),
    (
        b"Pacific/Tahiti",
        Item(
            tzdata::PACIFIC_TAHITI,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_TAHITI,
        ),
    ),
    (
        b"Brazil/East",
        Item(
            tzdata::AMERICA_SAO_PAULO,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_SAO_PAULO,
        ),
    ),
    (
        b"Pacific/Port_Moresby",
        Item(
            tzdata::PACIFIC_PORT_MORESBY,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_PORT_MORESBY,
        ),
    ),
    (
        b"Antarctica/Mawson",
        Item(
            tzdata::ANTARCTICA_MAWSON,
            #[cfg(feature = "binary")]
            raw_tzdata::ANTARCTICA_MAWSON,
        ),
    ),
    (
        b"Antarctica/McMurdo",
        Item(
            tzdata::ANTARCTICA_MC_MURDO,
            #[cfg(feature = "binary")]
            raw_tzdata::ANTARCTICA_MC_MURDO,
        ),
    ),
    (
        b"Atlantic/South_Georgia",
        Item(
            tzdata::ATLANTIC_SOUTH_GEORGIA,
            #[cfg(feature = "binary")]
            raw_tzdata::ATLANTIC_SOUTH_GEORGIA,
        ),
    ),
    (
        b"Antarctica/Macquarie",
        Item(
            tzdata::ANTARCTICA_MACQUARIE,
            #[cfg(feature = "binary")]
            raw_tzdata::ANTARCTICA_MACQUARIE,
        ),
    ),
    (
        b"Pacific/Fakaofo",
        Item(
            tzdata::PACIFIC_FAKAOFO,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_FAKAOFO,
        ),
    ),
    (
        b"Pacific/Norfolk",
        Item(
            tzdata::PACIFIC_NORFOLK,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_NORFOLK,
        ),
    ),
    (
        b"Africa/Windhoek",
        Item(
            tzdata::AFRICA_WINDHOEK,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_WINDHOEK,
        ),
    ),
    (
        b"Asia/Kuching",
        Item(
            tzdata::ASIA_KUCHING,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_KUCHING,
        ),
    ),
    (
        b"Europe/Tallinn",
        Item(
            tzdata::EUROPE_TALLINN,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_TALLINN,
        ),
    ),
    (
        b"Europe/Dublin",
        Item(
            tzdata::EIRE,
            #[cfg(feature = "binary")]
            raw_tzdata::EIRE,
        ),
    ),
    (
        b"Asia/Kabul",
        Item(
            tzdata::ASIA_KABUL,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_KABUL,
        ),
    ),
    (
        b"Asia/Oral",
        Item(
            tzdata::ASIA_ORAL,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_ORAL,
        ),
    ),
    (
        b"Antarctica/Palmer",
        Item(
            tzdata::ANTARCTICA_PALMER,
            #[cfg(feature = "binary")]
            raw_tzdata::ANTARCTICA_PALMER,
        ),
    ),
    (
        b"America/Halifax",
        Item(
            tzdata::AMERICA_HALIFAX,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_HALIFAX,
        ),
    ),
    (
        b"Europe/Gibraltar",
        Item(
            tzdata::EUROPE_GIBRALTAR,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_GIBRALTAR,
        ),
    ),
    (
        b"Asia/Novokuznetsk",
        Item(
            tzdata::ASIA_NOVOKUZNETSK,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_NOVOKUZNETSK,
        ),
    ),
    (
        b"Europe/Stockholm",
        Item(
            tzdata::EUROPE_STOCKHOLM,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_STOCKHOLM,
        ),
    ),
    (
        b"Asia/Karachi",
        Item(
            tzdata::ASIA_KARACHI,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_KARACHI,
        ),
    ),
    (
        b"Asia/Phnom_Penh",
        Item(
            tzdata::ASIA_PHNOM_PENH,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_PHNOM_PENH,
        ),
    ),
    (
        b"Atlantic/Cape_Verde",
        Item(
            tzdata::ATLANTIC_CAPE_VERDE,
            #[cfg(feature = "binary")]
            raw_tzdata::ATLANTIC_CAPE_VERDE,
        ),
    ),
    (
        b"Pacific/Midway",
        Item(
            tzdata::PACIFIC_MIDWAY,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_MIDWAY,
        ),
    ),
    (
        b"America/Kralendijk",
        Item(
            tzdata::AMERICA_CURACAO,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_CURACAO,
        ),
    ),
    (
        b"Europe/Tiraspol",
        Item(
            tzdata::EUROPE_CHISINAU,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_CHISINAU,
        ),
    ),
    (
        b"Africa/Brazzaville",
        Item(
            tzdata::AFRICA_BRAZZAVILLE,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_BRAZZAVILLE,
        ),
    ),
    (
        b"Europe/Istanbul",
        Item(
            tzdata::TURKEY,
            #[cfg(feature = "binary")]
            raw_tzdata::TURKEY,
        ),
    ),
    (
        b"Pacific/Chuuk",
        Item(
            tzdata::PACIFIC_CHUUK,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_CHUUK,
        ),
    ),
    (
        b"Atlantic/St_Helena",
        Item(
            tzdata::ATLANTIC_ST_HELENA,
            #[cfg(feature = "binary")]
            raw_tzdata::ATLANTIC_ST_HELENA,
        ),
    ),
    (
        b"Asia/Barnaul",
        Item(
            tzdata::ASIA_BARNAUL,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_BARNAUL,
        ),
    ),
    (
        b"Pacific/Honolulu",
        Item(
            tzdata::PACIFIC_HONOLULU,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_HONOLULU,
        ),
    ),
    (
        b"Africa/Kinshasa",
        Item(
            tzdata::AFRICA_KINSHASA,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_KINSHASA,
        ),
    ),
    (
        b"Asia/Jerusalem",
        Item(
            tzdata::ISRAEL,
            #[cfg(feature = "binary")]
            raw_tzdata::ISRAEL,
        ),
    ),
    (
        b"Pacific/Johnston",
        Item(
            tzdata::PACIFIC_HONOLULU,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_HONOLULU,
        ),
    ),
    (
        b"Australia/Perth",
        Item(
            tzdata::AUSTRALIA_PERTH,
            #[cfg(feature = "binary")]
            raw_tzdata::AUSTRALIA_PERTH,
        ),
    ),
    (
        b"Chile/Continental",
        Item(
            tzdata::AMERICA_SANTIAGO,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_SANTIAGO,
        ),
    ),
    (
        b"Europe/Uzhgorod",
        Item(
            tzdata::EUROPE_KIEV,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_KIEV,
        ),
    ),
    (
        b"Europe/Skopje",
        Item(
            tzdata::EUROPE_SKOPJE,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_SKOPJE,
        ),
    ),
    (
        b"Pacific/Palau",
        Item(
            tzdata::PACIFIC_PALAU,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_PALAU,
        ),
    ),
    (
        b"Europe/Simferopol",
        Item(
            tzdata::EUROPE_SIMFEROPOL,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_SIMFEROPOL,
        ),
    ),
    (
        b"Australia/Sydney",
        Item(
            tzdata::AUSTRALIA_ACT,
            #[cfg(feature = "binary")]
            raw_tzdata::AUSTRALIA_ACT,
        ),
    ),
    (
        b"Pacific/Pohnpei",
        Item(
            tzdata::PACIFIC_POHNPEI,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_POHNPEI,
        ),
    ),
    (
        b"Pacific/Yap",
        Item(
            tzdata::PACIFIC_CHUUK,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_CHUUK,
        ),
    ),
    (
        b"Etc/Zulu",
        Item(
            tzdata::UCT,
            #[cfg(feature = "binary")]
            raw_tzdata::UCT,
        ),
    ),
    (
        b"Europe/Minsk",
        Item(
            tzdata::EUROPE_MINSK,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_MINSK,
        ),
    ),
    (
        b"Pacific/Truk",
        Item(
            tzdata::PACIFIC_CHUUK,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_CHUUK,
        ),
    ),
    (
        b"Asia/Krasnoyarsk",
        Item(
            tzdata::ASIA_KRASNOYARSK,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_KRASNOYARSK,
        ),
    ),
    (
        b"Africa/Khartoum",
        Item(
            tzdata::AFRICA_KHARTOUM,
            #[cfg(feature = "binary")]
            raw_tzdata::AFRICA_KHARTOUM,
        ),
    ),
    (
        b"Asia/Baghdad",
        Item(
            tzdata::ASIA_BAGHDAD,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_BAGHDAD,
        ),
    ),
    (
        b"Universal",
        Item(
            tzdata::UCT,
            #[cfg(feature = "binary")]
            raw_tzdata::UCT,
        ),
    ),
    (
        b"America/Yellowknife",
        Item(
            tzdata::AMERICA_YELLOWKNIFE,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_YELLOWKNIFE,
        ),
    ),
    (
        b"Europe/Jersey",
        Item(
            tzdata::EUROPE_JERSEY,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_JERSEY,
        ),
    ),
    (
        b"Atlantic/Canary",
        Item(
            tzdata::ATLANTIC_CANARY,
            #[cfg(feature = "binary")]
            raw_tzdata::ATLANTIC_CANARY,
        ),
    ),
    (
        b"Atlantic/Stanley",
        Item(
            tzdata::ATLANTIC_STANLEY,
            #[cfg(feature = "binary")]
            raw_tzdata::ATLANTIC_STANLEY,
        ),
    ),
    (
        b"Europe/Kiev",
        Item(
            tzdata::EUROPE_KIEV,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_KIEV,
        ),
    ),
    (
        b"Europe/Ljubljana",
        Item(
            tzdata::EUROPE_LJUBLJANA,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_LJUBLJANA,
        ),
    ),
    (
        b"Antarctica/Troll",
        Item(
            tzdata::ANTARCTICA_TROLL,
            #[cfg(feature = "binary")]
            raw_tzdata::ANTARCTICA_TROLL,
        ),
    ),
    (
        b"Greenwich",
        Item(
            tzdata::GMT,
            #[cfg(feature = "binary")]
            raw_tzdata::GMT,
        ),
    ),
    (
        b"Europe/Bucharest",
        Item(
            tzdata::EUROPE_BUCHAREST,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_BUCHAREST,
        ),
    ),
    (
        b"Asia/Kathmandu",
        Item(
            tzdata::ASIA_KATHMANDU,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_KATHMANDU,
        ),
    ),
    (
        b"Europe/Kirov",
        Item(
            tzdata::EUROPE_KIROV,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_KIROV,
        ),
    ),
    (
        b"Europe/Chisinau",
        Item(
            tzdata::EUROPE_CHISINAU,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_CHISINAU,
        ),
    ),
    (
        b"Europe/Zurich",
        Item(
            tzdata::EUROPE_BUSINGEN,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_BUSINGEN,
        ),
    ),
    (
        b"Antarctica/Casey",
        Item(
            tzdata::ANTARCTICA_CASEY,
            #[cfg(feature = "binary")]
            raw_tzdata::ANTARCTICA_CASEY,
        ),
    ),
    (
        b"Asia/Bishkek",
        Item(
            tzdata::ASIA_BISHKEK,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_BISHKEK,
        ),
    ),
    (
        b"Etc/Universal",
        Item(
            tzdata::UCT,
            #[cfg(feature = "binary")]
            raw_tzdata::UCT,
        ),
    ),
    (
        b"Asia/Thimphu",
        Item(
            tzdata::ASIA_THIMBU,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_THIMBU,
        ),
    ),
    (
        b"Turkey",
        Item(
            tzdata::TURKEY,
            #[cfg(feature = "binary")]
            raw_tzdata::TURKEY,
        ),
    ),
    (
        b"Antarctica/Vostok",
        Item(
            tzdata::ANTARCTICA_VOSTOK,
            #[cfg(feature = "binary")]
            raw_tzdata::ANTARCTICA_VOSTOK,
        ),
    ),
    (
        b"America/La_Paz",
        Item(
            tzdata::AMERICA_LA_PAZ,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_LA_PAZ,
        ),
    ),
    (
        b"Australia/LHI",
        Item(
            tzdata::AUSTRALIA_LHI,
            #[cfg(feature = "binary")]
            raw_tzdata::AUSTRALIA_LHI,
        ),
    ),
    (
        b"Europe/Ulyanovsk",
        Item(
            tzdata::EUROPE_ULYANOVSK,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_ULYANOVSK,
        ),
    ),
    (
        b"Atlantic/Azores",
        Item(
            tzdata::ATLANTIC_AZORES,
            #[cfg(feature = "binary")]
            raw_tzdata::ATLANTIC_AZORES,
        ),
    ),
    (
        b"Asia/Sakhalin",
        Item(
            tzdata::ASIA_SAKHALIN,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_SAKHALIN,
        ),
    ),
    (
        b"Pacific/Wallis",
        Item(
            tzdata::PACIFIC_WALLIS,
            #[cfg(feature = "binary")]
            raw_tzdata::PACIFIC_WALLIS,
        ),
    ),
    (
        b"Asia/Irkutsk",
        Item(
            tzdata::ASIA_IRKUTSK,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_IRKUTSK,
        ),
    ),
    (
        b"Asia/Kuala_Lumpur",
        Item(
            tzdata::ASIA_KUALA_LUMPUR,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_KUALA_LUMPUR,
        ),
    ),
    (
        b"Portugal",
        Item(
            tzdata::PORTUGAL,
            #[cfg(feature = "binary")]
            raw_tzdata::PORTUGAL,
        ),
    ),
    (
        b"Europe/Guernsey",
        Item(
            tzdata::EUROPE_GUERNSEY,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_GUERNSEY,
        ),
    ),
    (
        b"etc/localtime",
        Item(
            tzdata::FACTORY,
            #[cfg(feature = "binary")]
            raw_tzdata::FACTORY,
        ),
    ),
    (
        b"Hongkong",
        Item(
            tzdata::HONGKONG,
            #[cfg(feature = "binary")]
            raw_tzdata::HONGKONG,
        ),
    ),
    (
        b"Atlantic/Reykjavik",
        Item(
            tzdata::ICELAND,
            #[cfg(feature = "binary")]
            raw_tzdata::ICELAND,
        ),
    ),
    (
        b"Europe/Kyiv",
        Item(
            tzdata::EUROPE_KIEV,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_KIEV,
        ),
    ),
    (
        b"Asia/Riyadh",
        Item(
            tzdata::ASIA_RIYADH,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_RIYADH,
        ),
    ),
    (
        b"Europe/Vaduz",
        Item(
            tzdata::EUROPE_VADUZ,
            #[cfg(feature = "binary")]
            raw_tzdata::EUROPE_VADUZ,
        ),
    ),
    (
        b"Mexico/General",
        Item(
            tzdata::AMERICA_MEXICO_CITY,
            #[cfg(feature = "binary")]
            raw_tzdata::AMERICA_MEXICO_CITY,
        ),
    ),
    (
        b"Etc/Greenwich",
        Item(
            tzdata::GMT,
            #[cfg(feature = "binary")]
            raw_tzdata::GMT,
        ),
    ),
    (
        b"Asia/Yakutsk",
        Item(
            tzdata::ASIA_YAKUTSK,
            #[cfg(feature = "binary")]
            raw_tzdata::ASIA_YAKUTSK,
        ),
    ),
];

const ASSO_VALUES: [u16; 257] = [
    2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324,
    2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324,
    2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 5, 5, 7, 10, 744, 5, 20, 17,
    32, 30, 12, 25, 10, 15, 26, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 5, 43, 118, 23, 10, 369,
    39, 729, 26, 218, 432, 543, 232, 5, 10, 272, 82, 15, 86, 6, 190, 238, 147, 64, 496, 612, 20,
    2324, 2324, 2324, 326, 5, 5, 43, 118, 23, 10, 369, 39, 729, 26, 218, 432, 543, 232, 5, 10, 272,
    82, 15, 86, 6, 190, 238, 147, 64, 496, 612, 20, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324,
    2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324,
    2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324,
    2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324,
    2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324,
    2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324,
    2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324,
    2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324,
    2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324, 2324,
];

fn find_item(s: &[u8]) -> Option<&'static Item> {
    let len = s.len();
    if !matches!(len, 2..=25) {
        return None;
    }

    let mut key: usize = len;
    key = key.wrapping_add(ASSO_VALUES[s[len - 1] as usize] as usize);
    if len > 11 {
        key = key.wrapping_add(ASSO_VALUES[s[11] as usize] as usize);
    }
    if len >= 11 {
        key = key.wrapping_add(ASSO_VALUES[s[10] as usize] as usize);
    }
    if len >= 9 {
        key = key.wrapping_add(ASSO_VALUES[s[8] as usize] as usize);
    }
    if len >= 8 {
        key = key.wrapping_add(ASSO_VALUES[s[7].wrapping_add(1) as usize] as usize);
    }
    if len >= 6 {
        key = key.wrapping_add(ASSO_VALUES[s[5].wrapping_add(1) as usize] as usize);
    }
    if len >= 4 {
        key = key.wrapping_add(ASSO_VALUES[s[3] as usize] as usize);
    }
    if len >= 2 {
        key = key.wrapping_add(ASSO_VALUES[s[1].wrapping_add(1) as usize] as usize);
    }
    if len >= 1 {
        key = key.wrapping_add(ASSO_VALUES[s[0] as usize] as usize);
    }

    if key > 2323 {
        return None;
    }
    let key = WORDLIST[key]?;
    let key: u16 = unsafe { transmute(key) };
    let (key, ref item) = ITEMS[key as usize];
    if !key.eq_ignore_ascii_case(s) {
        return None;
    }

    Some(item)
}

pub(crate) fn find_tz(s: &[u8]) -> Option<TimeZoneRef<'static>> {
    Some(find_item(s)?.0)
}

#[cfg(feature = "binary")]
pub(crate) fn find_raw(s: &[u8]) -> Option<&'static [u8]> {
    Some(find_item(s)?.1)
}
