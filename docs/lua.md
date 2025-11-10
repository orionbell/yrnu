# Lua

Lua is a fast and easy to use programing language design to be minimalistic as possible

Here is a 5 min Lua tutorial to get you started, for more deep dive tutorial consider [this awesome site](https://learnxinyminutes.com/lua/)

## Variables
```lua
var = "value"       -- global variable
local var = "value" -- local variable
```
### Data types

#### Number
```lua
-- In lua float and int are the same type and their type referd as number
bar = 10
PI = 3.1415
```
#### Bool
```lua
foo = true
zas = false
```
#### String
```lua
-- Lua strings are immutable
var = 'Hi mom'       
foo = "Hello World"  
dor = [[             
    multi line
    strings
]]
```
#### Table
```lua
-- In lua Tables are the only data structure.
-- It can be array or hashmap (a.k.a Objects or Dictineries)  
-- In lua array indexes starts at 1
arr = {"A","b",3,4,5} -- same as {1 = "A", 2 = "b", 3 = 3, 4 = 4, 5 = 5}

print(arr[1]) -- "A"

arr[1] = "B"
map = { name = "John", age = 22 }

print(map.name) -- John

map["name"] = "Mike"
map.age = 18
```
#### Nil
```lua
user = nil -- same as null or none in other languages

-- In lua every time we try to access undefined variable or field its value is nil
print(map.not_exists) -- nil 
```

## Conditions
```lua
if is_cool and not name == "John" then
    print("Hi John")
elseif num >= 45 or name ~> "Mike" then
    print("Hi Mike")
else
    print("Who are you?")
end
```
#### One line conditions
```lua
if is_cool then print("Hi mom") end

var = is_cool and "Cool" or "NotCool" -- (is_cool and "Cool") or "NotCool"

```
## Loops
#### While Loop
```lua
while num > 20 do
    num-=1
end
```
#### For Loop
```lua
-- start = 1, end = 10
for i = 1, 10 do 
    num += i
end
-- start = 10, end = 0, step = -2 - step is optional and defaults to 1
for j = 10, 0, -2 do 
    num *= j
end

for j = 1, 5 do print("Do I?") end
```
##### Foreach Loop
```lua
nums = {1,2,3,4}
for i in pairs(nums) do 
    num += i             -- 1 2 3 4
end
person = { name = "Tom", age = 25, pet_name = "tiger" }
for k,v in pairs(person) do print(k .. " : " .. v) end
-- name : Tom
-- age : 25 
-- pet_name : tiger
```
#### Until Loop
```lua
repeat
    print(num)
    num -=1
until num == 0
```
## Functions

```lua
function add(x,y)
    return x + y
end

sub = function (x,y) return x - y end

print(add(1,5)) -- 6
print(sub(5,1)) -- 4
```

#### Functions in tables
```lua
cat = { name = "anakin", owner_name = "Josh" }
cat.make_sound = function (name) print(name .. " is Meowing") end
print(cat.make_sound(cat.name)) -- "anaking is Meowing"
```
A better implementation would be by using the `:` operator instend of the `.`, When doing so Lua auto passing the table as the first argument to the function
```lua{2,3}
cat = { name = "anakin", owner_name = "Josh" }
cat.make_sound = function (name) print(name .. " is Meowing") end -- [!code --]
print(cat.make_sound(cat.name)) -- "anaking is Meowing" [!code --]
cat.make_sound = function (self) print(self.name .. " is Meowing") end -- [!code ++]
print(cat:make_sound()) -- "anaking is Meowing" [!code ++]
print(cat.make_sound(cat)) -- same as above
```
In general functions that accept self refer to as ***methods***

---

Now that you feel comfortable in using Lua let's start

