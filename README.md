# Phone number parser
### by Anton Pysmennyi

#### This small parser will help you with some basic operations regarding the phone numbers. 
#### This includes:
<ul>
    <li> format conversion (to E.164) </li>
    <li> country codes extraction </li>
    <li> operator extraction </li>
</ul>

#### Here are currently supported phone number formats: 
<ul>
    <li> +380 44 1231231 </li>
    <li> +380 (44) 1231231 </li>
    <li> +380 50 123 4567 </li>
    <li> 00 380 (50) 123 4567 </li>
</ul>

#### This list can be expanded in the future updates


## Installation
The technology requires to use Rust developer stack

After it's installed, clone this repository to your machine:
```
git clone https://github.com/BlVrp/pysmennyi_phone_parser.git
```
After cloning, build the project's release version.
```
cd pysmennyi_phone_parser
cargo build --release
```

And just like that you have a working executable. Feel free to use it: the guide is right below this line :)
## How to use?
If you are confused about the phrasing of this document, you can always consult the output of the help command, perhaps you will find it more helpful
```
.\pysmennyi_phone_parser help
```

The command line interface supports 3 modes:
<ul>
    <li>E.164 conversion (drop all separators, essentially)</li>
    <li>country codes extraction</li>
    <li>operator codes extraction</li>
</ul>

### Expected input format
The cli expects the file with one phone number per row as its input 
```
+38 099 999 99 99
+1 (223) 12345
00 8 (23) 22 22
```

### E.164 conversion
Execute the command following this pattern to read phone numbers from the file, convert them to E.164 and write them to the output file. Keep in mind: the output file will be overriden:

```
.\pysmennyi_phone_parser "./input.txt" "./output.txt" convert-to-e164
```
input:
```
+38 099 999 99 99
+1 (223) 12345
00 8 (23) 22 22
```
output sample: 
```
+3809999999
+122312345
+8232222
```

The other two modes are very similar in terms of usage:
### Operator codes extraction
```
.\pysmennyi_phone_parser "./input.txt" "./output.txt" extract-operator-codes
```
input:
```
+38 099 999 99 99
+1 (223) 12345
00 8 (23) 22 22
```
output sample: 
```
099
223
23
```
### Country codes extraction
```
.\pysmennyi_phone_parser "./input.txt" "./output.txt" extract-country-codes
```
input:
```
+38 099 999 99 99
+1 (223) 12345
00 8 (23) 22 22
```
output sample: 
```
38
1
8
```
## Grammar explained
Considering the fact that the project might grow in the future, with more formats adding up, the goal is to make the rules as readable and as maintainable as possible
That's why some redundancy might be seen here and there.
Because of that, the following approach is used:
<br>
#### For each phone number format there is a meta-rule that describes it
#### those meta-rules are composed of reusable (somewhat) small rules

### Here are a few examples of phone format rules:
```
international_with_spaces = {plus ~ country_code ~ whitespace ~ operator_code ~ local_number_block}
international_with_braces = {plus ~ country_code ~ whitespace ~ "(" ~ operator_code ~ ")" ~ local_number_block}
```

as we can see, each of those utilizes smaller rules which repeat themselves:
```
whitespace = {" "}
plus = {"+"}
double_zero = {"00"}
operator_code = {ASCII_DIGIT{2,5}}
country_code = {ASCII_DIGIT{1,3}}
local_number_block = {group_separator ~ ASCII_DIGIT{5,10}}
group_separator = {"-" | " "}
local_number_group = {group_separator ~ ASCII_DIGIT{2,4}}
```



## Licencing

#### don't bother, use as you wish :)

