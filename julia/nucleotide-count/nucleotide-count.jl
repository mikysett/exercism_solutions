"""
    count_nucleotides(strand)

The count of each nucleotide within `strand` as a dictionary.

Invalid strands raise a `DomainError`.

"""
function count_nucleotides(strand)
    count = Dict((n, 0) for n in "ACGT")
    for c in strand
        c in "ACGT" || throw(DomainError(c))
        count[c] += 1
    end
    count
end
