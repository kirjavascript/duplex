// used in trainer + subsets
//
// 10:23 <+Kirjava> I know how to enumerate when the mask is just permutation
// 10:24 <+Kirjava> but orientation is a tricky one
// 10:24 <+Kirjava> maybe I do two phase enumeration
//
// get CLL, edge lsit of edges for swap and no swap
// mask -> get list of indexes
//
// send mask back to respond with results

list alg coverage /add mirror F/B

trim AUF for Vec<Move>
give algs names w/ colours
click to see the alg / auto mirrored -> show middle case

rotate cube LL UI

implement solving mask Face::Null
subset chooser like 2LLui - show coverage
subsets - strip first auf by rotating
select subset, generate cases that use smallest number of first algs OR shortest OR minimize auf
<a>all solutions</a>
<a>alg</a> toggle alg / name
show inbetween state for solution
LLs like sprites from flex -group by first alg

<+Kirjava> that's something I'm wondering about DXLL. can you learn rules that allow you to solve cases you haven't learnt in a one by one fashion
<+Kirjava> like something that applies to a group of cases
SPA mode website

> duplex on cll

* add ability to select a subset, and list solutions that use the smallest quantity of first alg -> this is how to find recog patterns -> add colours to tell algs apart
* show shortest, reduce first alg

for algs, look at OLL/CMLL
subset chooser (filter)
gamification (checkbox for learnt subset / case) |
2x2x3 left as exercise for the reader |
dxll is a one look last layer method |
modernization of the petrus method |
done before: 270 / OLLCP
recommend cases / list recommended
no bad algs™
leaderboard / worst case
stats / users / main

G:
0 [F, U, R, U', R', F'] 2 [F', L', U, L', D', L, U2, L', D, L2, U, F]
0 [F', U', L2, D', L, U2, L', D, L, U', L, F] 2 [F, R, U, R', U', F']
