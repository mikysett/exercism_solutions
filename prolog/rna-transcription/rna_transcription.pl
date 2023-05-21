rna_transcription(RnaStr, DnaStr):-
    string_chars(RnaStr, Rna),
    rna_trans(Rna, Dna),
    string_codes(DnaStr, Dna).

rna_trans([], []).
rna_trans([H|T], [Nh|Nt]):-
    trans(H, Nh),
    rna_trans(T, Nt).

trans('G', 'C').
trans('C', 'G').
trans('T', 'A').
trans('A', 'U').
