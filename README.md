# Sap lang
# **🚧upgrading to v2**
> yet another configuration oriented language
> 
> name comes from Sapphire which is the birthstone of september

## Language Feature
the last expr of the scope is the return value
also the last expr of the file is the return value of the file

## variable decl
variable is scoped
```
a = 10
b = 20
c = "string"
d = 'c' # TODO: number in char 
```

## comment
TODO
```
# single lined comment

###BEGIN
multi
lined 
comment
###END
```

## literals
```
10 # number
'c' # number in char literal
"string" # string

##RAW
raw string
raw string
##

# TODO
#INCLUDE "path" # also raw string

true # bool
[] # array
{a:10,b:20} # object
(){null} # closure

# fast object
a = 10
{a} # semantic: {a:a}
```

## strong type?
YES!

## type definision
```
type typed = (name){{name}}

tyvar = typed(name)

type enum = 
    | name: (name){{name}}
    | aged: (name,age){{name,age}}
    | full: (name,age,location){{name,age,location}}

tyvar = enum(name,age,location)
# or
tyvar = enum::full(name,age,location)

# change value
tyvar[name] = "string"

# to validate if is a variant of an enum

TODO
```

## function
```
# auto typed
(a){a}

# typed
(a:String){a}
```

pre defined function
- log # more or less println
- octor
- actor
- is # try to match object enntrys with function argument name
- assert
- intersect union diff cross
- innerjoin  joinon


## branch
```
if expr:bool {

} else {

}

if
    | expr:bool {}
    | expr:bool {}
    | expr:bool {}
else {}
```

## looping
```
for a in [] {

}

for a in range(0,10){

}
```

## function call
```
f = (a:number) => number {a.s}
g = (a:string) => number {a.n?}

f(10) # "10"
g("10") # 10
f.g.f.g.f.g.f(10) # 10 . is bind

id = f.g
```

## error handling
```
a = null
a # return file end as object
# so calling a.o
# {}

# another file
b = err("an error")
b
# error in line 1 b: an error

b = err("an error")? # cast error to null
b

b = if maybeerr? == null {
    1
}else{
    maybeerr?
}
```

## dot sematic
```
a.b # a: Object b:Any => index
a.b # a: function b:function => bind
a.b() # a: not function b: function => bind first
a().b() # a: function call with less param b: function call with multiple param but less one => bind first
a.typename() # cast 
```

## function semantic
function is curried by default

## Example File
```
url = ARGS.url.orelse("https://lemonhx.moe")
port = ARGS.url.orelse(8080)

type Request = (post_name,post_id){
    {
        type: "POST"
        url: "{{url}}/{{post_name}}/{{post_id}}",
        port,
    }
};

Request("blogexample",10)
```

then you can use like

```
sap example.sap -serializer json url "https://google.com" -o stdout

{
    "type":"POST",
    "url":"https://google.com/blogexample/10",
    "port":8080
}

sap example.sap -serializer yaml url "https://baidu.com" -o stdout

type: POST
url: 'https://google.com/blogexample/10'
port: 8080
```
