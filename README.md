# Z-sec Lang
![_f8f04183-2732-42a8-933a-509edef48102-removebg-preview(2)](https://github.com/user-attachments/assets/5feb85b1-f1c4-454c-8263-9de37c4b13be)
### Version 0.01

[+] Integer ve string tipinde değişken oluşturma ve değişkenin değerini bellekte tutma. 
```
int->var_nam1=12;
string->var_name2="foo";
```
[+] Oluşturulan değerin tipini algılayabilme (Şuanlik değişkeni bellekte tutmuyor)
```
bool->var_name2=12;
float->var_name3=3.2;
```
![zsec1](https://github.com/user-attachments/assets/1865d865-c407-4a62-a1cd-a9db9648c702)

[+] Fonksiyonu algılama (Fonksiyon işlevini yerine getirmiyor.)
```
func func_name -> int() //Döndürülecek değerin tipi
{
}
```

### Version 0.02

[+] Artık boolean tipinde de değişken oluşturabilme, syntax kontrolü yapabilme ve bellekte tutabilme
```
bool->var_name2=true;
```
[+] String değişkeni tanımlanırken syntax kontrolü
```
string->var_name4="foo";
```
![zsec2](https://github.com/user-attachments/assets/afb3bd12-1465-419f-8521-fa2a15c352a7)

[!] Update

  Bellek kullanımında iyileştirmeler yapıldı.
### Version 0.03
#### Değişiklikler:
[+] Daha güvenli bellek kontrolü için yazılma dili C++'dan Rust'a çevrildi.

[+]Int, float veri tipi ve tip kontrolü

[+]"print" fonksiyonu

[+] Hata ayıklama

[!] Syntax değişikliği.

##### _Çıktılar_
##### _1._
```
int :> a = 23;
float :> b=3.2;
print(b);
```
<img width="822" height="72" alt="1" src="https://github.com/user-attachments/assets/acd9245a-be24-4e82-98e1-ef50c0214809" />

##### _2._

```
int :> a = 23;
float :> b=3.2;
print(c);
int :> c=2;
```

_c değişkeni oluşturulmadığı için beklenen hata, hatanın oluştuğu satırı da bildiriyor:_

<img width="771" height="93" alt="5" src="https://github.com/user-attachments/assets/b190adb0-6a3a-4a86-927c-596f8f8b727d" />

##### _3._

```
int :> a = 23;
float :> b=3.2;

print("b");
print(b);
print("qwert");
print("qwert123");
```

<img width="736" height="120" alt="6" src="https://github.com/user-attachments/assets/ba3acf8b-347f-40d7-9a41-0586e4967b7f" />

#### _4._
```
int :> a = 23;
float :> b=3.2;

prindt("b");
```

<img width="741" height="88" alt="7" src="https://github.com/user-attachments/assets/05375598-7cfb-4374-b54a-49d600ff8ef4" />



