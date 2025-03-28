Autokey ciphers use the plain text (shifted with the initial keyword) too encode the message, but we know how the flags begin (utflag).

We can figure out the first 6 letters of the intial keyword just working backwards since we know the plain text. We get "rwllmu" as our intial keyword. (We see that it hasn't started repeating so it must be at least 6 letters).

From here we can decode at various lengths (just use a filler value for letters over 6), and we get the following.

rwllmu: utflag{xdn_nwjtxnheu_yudyylmw_twgn_wjds_ryomuiflt_dqtbggc}
rwllmux: utflag{ucz_tlpkgyvhs_chkhhyso_pkxr_kniz_notlksxwt_wzroudm}
rwllmuxx: utflag{uzy_frequgvcy_analwkis_when_mvow_begilfing_letvmrs}

With 8 characters, we start to see something resembling words. We can guess the correct letters from here. (If you can't, you only need to guess the correct letters in one place, and you can work backwords to get the initial keyword).
