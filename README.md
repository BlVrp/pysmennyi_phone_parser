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
