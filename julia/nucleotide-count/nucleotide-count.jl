"""
    count_nucleotides(strand)

The count of each nucleotide within `strand` as a dictionary.

Invalid strands raise a `DomainError`.

"""
function count_nucleotides(strand)
    count = Dict('A' => 0, 'C' => 0, 'G' => 0, 'T' => 0)
    for c in strand
        if !occursin(string(c), "ACGT")
            throw(DomainError(c))
        else
            count[c] += 1
        end
    end
    count
end
