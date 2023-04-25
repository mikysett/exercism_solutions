function raindrops(number)
    result = ""
    if number % 3 == 0
        result = "Pling"
    end
    if number % 5 == 0
        result = string(result, "Plang")
    end
    if number % 7 == 0
        result = string(result, "Plong")
    elseif result == ""
        result = string(number)
    end
    result
end
