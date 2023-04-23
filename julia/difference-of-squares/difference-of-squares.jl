"Square the sum of the first `n` positive integers"
square_of_sum(n) = sum(1:n)^2

"Sum the squares of the first `n` positive integers"
sum_of_squares(n) = sum((1:n).^2)

"Subtract the sum of squares from square of the sum of the first `n` positive ints"
function difference(n)
    square_of_sum(n) - sum_of_squares(n)
end
