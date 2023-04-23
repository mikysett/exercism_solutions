"""
    ispangram(input)

Return `true` if `input` contains every alphabetic character (case insensitive).

"""
function ispangram(input)
    foundLetters = ""
    for c in lowercase(input)
        if isletter(c) && !contains(foundLetters, string(c))
            foundLetters = string(foundLetters, c)
        end
    end
    length(foundLetters) == 26
end

