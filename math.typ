#let inv(n) = math.frac([1],n);

= Bogosort

Bogosort is one of many array sorting algorithms. For many algorithms, the effectiveness and time complexity are fairly easy to determine, because most algorithms are deterministic. That means that the steps the algorithm will take to sort the array can be predicted with a success rate of 1. Bogosort however, is not deterministic.

Bogosort is a very simple algorithm. Take an array, and randomise its contents. If the array is sorted, then stop. If the array isn't, repeat.

This means that bogosort is simultaniously the best and worst sorting algorithm. It could sort an array with a million items within ten iterations, but it could also take a million iterations to sort an array of ten elements.

This paper aims to use probability to research the complexity of bogosort.

== $(l,n)->p$

Firstly, we'll calculate the chance that bogosort will have sorted an array of $l$ elements after $n$ iterations, given there is only one configuration we count as "sorted".

$
  P(l, n) & = sum_(k=1)^n (1-1/l!)^(k-1) dot 1/l! \
          & = 1/l! sum_(k=1)^n (1-1/l!)^(k-1) \
          & = 1/l! sum_(k=0)^(n-1) (1-1/l!)^k \
          & = 1/l! (frac(1-(1-1/l!)^n, 1-(1-1/l!))) \
          & = 1-(1-1/l!)^n \
$

== $(l,p)->n$

Secondly, we'll focus on the reverse. Given a certain probability $p$, how many iterations does it take for the probability that an array of $l$ elements will be sorted to exceed $p$? We can determine this by taking the inverse function of $P(l,n)$, keeping $l$ constant.
$
          P(l,N(l,p)) & = p \
  1-(1-1/l!)^(N(l,p)) & = p \
                  1-p & = (1-1/l!)^(N(l,p)) \
              ln(1-p) & = N(l,p) ln(1-1/l!) \
               N(l,p) & = frac(ln(1-p), ln(1-1/l!)) \
               N(l,p) & = log_((1-1/l!))(1-p) \
$

This helps us give an estimate of the effectiveness of bogosort.

=== Examples

#table(
  columns: 5,
  align: horizon + center,
  [$ l $], [$ p=0.1 $], [$ p=0.5 $], [$ p=0.9 $], [$ p=0.99 $],
  [$ 1 $], [$ 0 $], [$ 0 $], [$ 0 $], [$ 0 $],
  [$ 2 $], [$ 1 $], [$ 1 $], [$ 4 $], [$ 7 $],
  [$ 3 $], [$ 1 $], [$ 4 $], [$ 13 $], [$ 26 $],
  [$ 4 $], [$ 3 $], [$ 17 $], [$ 55 $], [$ 109 $],
  [$ 5 $], [$ 13 $], [$ 83 $], [$ 276 $], [$ 551 $],
  [$ 6 $], [$ 76 $], [$ 499 $], [$ 1657 $], [$ 3314 $],
  [$ 7 $], [$ 531 $], [$ 3494 $], [$ 11604 $], [$ 23208 $],
  [$ 8 $], [$ 4249 $], [$ 27948 $], [$ 92840 $], [$ 185679 $],
);

As we can see from both the example values and the formula itself, the amount of tries required grows very fast. In fact, its growth is linearly proportional to $l!$.

== Approximation for $N(l,p)$

Before the author realised that the formula for $P(l,n)$ could be easily solved, they tried a different approach to find an approximation for said function. Eventually the following approximation was found:

$
  N(l,p) & approx ln(inv(1-p))l! \
$

Why this approximation works becomes clear when we utilize a famous approximations using reciprocals and the natural logarithm.

$
                               inv(l) & approx ln(l) - ln(l-1) \
                               inv(l) & approx ln(frac(l, l-1)) \
                              inv(l!) & approx ln(frac(l!, l!-1)) \
              inv(ln(frac(l!, l!-1))) & approx l! \
                 inv(ln(inv(1-1/l!))) & approx l! \
  frac(ln(inv(1-p)), ln(inv(1-1/l!))) & approx ln(inv(1-p))l! \
            frac(ln(1-p), ln(1-1/l!)) & approx ln(inv(1-p))l! \
                               N(l,p) & approx ln(inv(1-p))l! \
$

Using the approximation $1/l = ln(l)-ln(l-1)$, the approximation for $N(l,p)$ forms.
