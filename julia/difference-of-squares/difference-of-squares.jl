"Square the sum of the first `n` positive integers"
function square_of_sum(n)
    sum = 0
    for i in 1:n
        sum += i
    end
    sum * sum
end

"Sum the squares of the first `n` positive integers"
function sum_of_squares(n)
    sum = 0;
    for i in 1:n
        sum += i * i
    end
    sum
end

"Subtract the sum of squares from square of the sum of the first `n` positive ints"
function difference(n)
    square_of_sum(n) - sum_of_squares(n)
end
