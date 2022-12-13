sol1 =
  File.read("input.txt")
    .split("\n")
    .map{|e| e.chars.each_slice(e.size / 2).to_a }
    .map{|e| e[0]
    .intersection(e[1])[0]}
    .map(&:ord)
    .map{ |e| if e >= 96 then e - 96 else e - 38 end }
    .sum

print "Solution 1: #{sol1}"

sol2 =
  File.read("input.txt")
    .split("\n")
    .map(&:chars)
    .each_slice(3)
    .map{ |e| e[0].intersection(e[1]).intersection(e[2]).to_a[0] }
    .map(&:ord)
    .map{ |e| if e >= 96 then e - 96 else e - 38 end }
    .sum
    
print "Solution 2: #{sol2}"
