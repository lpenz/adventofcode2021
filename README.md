![AoC](https://img.shields.io/badge/AoC%20%E2%AD%90-50-yellow)
[![CI](https://github.com/lpenz/adventofcode2021/workflows/CI/badge.svg)](https://github.com/lpenz/adventofcode2021/actions)
[![coveralls](https://coveralls.io/repos/github/lpenz/adventofcode2021/badge.svg?branch=main)](https://coveralls.io/github/lpenz/adventofcode2021?branch=main)

# adventofcode2021

Code for the 2021 exercises at https://adventofcode.com/2021/


## Noteworthy days (spoiler alert!)

Some interesting things that happened on specific days:

- Day 08: 7-segment digit reconstruction.
- Day 10: delimiter matching.
- Day 12: depth-first search.
- Day 13: origami!
- Day 15: uniform-cost search (Dijkstra); puzzle B code pending - the
  grid is too big, we need to support different underlying structs in
  [sqrid].
- Day 16: binary parsing using [nom].
- Day 17: discrete parabolic trajectories.
- Day 19: linear transformations using matrices - and an actual list
  of all valid linear rotational matrices.
- Day 22: positive and negative area cuboids - reminds me of firewall
  rule analysis.
- Day 24: used [rayon] for parallelism and [dashset] for cache to get
  a viable brute-force performance.


<table><tr>
<td><a href="https://github.com/lpenz/adventofcode2020">:arrow_left: 2020</td>
</tr></table>

[sqrid]: https://github.com/lpenz/sqrid
[nom]: https://github.com/Geal/nom
[rayon]: https://github.com/rayon-rs/rayon
[dashset]: https://github.com/xacrimon/dashmap
