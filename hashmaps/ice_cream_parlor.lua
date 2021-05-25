function whatFlavors(costs, money)
    local map = {}
    for i, cost in ipairs(costs) do
        if map[money - cost] ~= nil then
            return math.min(map[money - cost], i), math.max(map[money-cost], i)
        else
            map[cost] = i
        end
    end
end

local t = io.stdin:read("*n", "*l")

for titr = 1, t do
    local money = io.stdin:read("*n", "*l")
    local n = io.stdin:read("*n", "*l")
    local cost = {}

    for token in string.gmatch(io.stdin:read("*l"):gsub("%s+$", ""), "[^%s]+") do
        table.insert(cost, tonumber(token))
    end

    print(whatFlavors(cost, money))
end
