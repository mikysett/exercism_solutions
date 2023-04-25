function raindrops(number)
    result = ""
    number % 3 == 0 && (result *= "Pling")
    number % 5 == 0 && (result *= "Plang")
    number % 7 == 0 && (result *= "Plong")
    isempty(result) ? string(number) : result
end
