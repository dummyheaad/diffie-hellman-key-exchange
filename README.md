# Diffie-Hellman Key Exchange

## Deskripsi
Diffie-Hellman Key Exchange merupakan sebuah algoritma yang digunakan dalam pertukaran kunci.  
Kunci yang dimaksud disini adalah kunci publik yang nantinya digunakan oleh 2 pihak untuk berkomunikasi secara aman.  
Algoritma ini akan memungkinkan 2 pihak berbagi kunci publik yang sama dan kunci publik ini dapat digunakan
untuk mengenkripsi pesan sebelum dikirimkan ke saluran publik berupa Internet.  
Kunci publik dibentuk dari 2 buah kunci privat yang dibangkitkan oleh masing-masing pihak.  
Perlu dipahami bahwa algoritma ini hanya digunakan untuk berbagi kunci publik saja.  
Algoritma ini tidak digunakan dalam proses enkripsi dan dekripsi pesan (tidak direkomendasikan).  
Oleh karena itu, Algoritma Diffie-Helman selalu dikombinasikan dengan algoritma enkripsi-dekripsi pesan lainnya seperti AES, RSA, Elgamal, dan lain-lain 
maupun protokol jaringan seperti TLS, SSH, dan VPN.

## Cara Kerja
### Skema
Alice dan Bob ingin menerapkan Diffie-Hellman Key Exchange  

### Input
- p => bilangan prima (besar)&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[tidak rahasia / publik]  
Nilai p dipilih secara acak dan disepakati oleh Alice dan Bob
  
- g => bilangan bulat generator (bisa besar namun umumnya kecil)&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[tidak rahasia / publik]  
Nilai g dipilih secara acak dan disepakati oleh Alice dan Bob  
**g harus primitive root modulo terhadap p**
  
- a => kunci privat Alice (2 <= a <= p - 2)&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[rahasia / privat]  
Nilai a dipilih secara acak oleh Alice

- A => kunci publik Alice&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[tidak rahasia/publik]  
$A\ =\ g^a\ mod\ p$  
Kunci publik Alice dihitung kemudian dikirimkan ke Bob

- b => kunci privat Bob (2 <= b <= p - 2)&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[rahasia / privat]  
Nilai b dipilih secara acak oleh Bob

- B => kunci publik Bob&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[tidak rahasia / publik]  
$B\ =\ g^b\ mod\ p$  
Kunci publik Bob dihitung kemudian dikirimkan ke Alice	

### Tahapan
1. Alice dan Bob menyepakati dua buah bilangan, yaitu p dan g  
Bilangan p merupakan sebuah bilangan prima dan g merupakan sebuah bilangan bulat yang primitive root modulo terhadap p.

2. Alice memilih bilangan bulat acak a sebagai kunci privatnya kemudian melakukan perhitungan
kunci publik A. Setelah itu, Alice mengirimkan kunci publik A kepada Bob.

3. Bob memilih bilangan bulat acak b sebagai kunci privatnya kemudian melakukan perhitungan
kunci publik B. Setelah itu, Bob mengirimkan kunci publik B kepada Alice.

4. Alice menerima kunci publik B dari Bob dan kemudian menghitung K.

5. Bob menerimam kunci publik A dari Alice dan kemudian menghitung K.

6. Alice dan Bob sekarang berbagi kunci publik yang sama, yaitu K.

### Output
K => kunci publik yang bersifat shared (berbagi bersama) antara Alice dan Bob
Nilai K dihitung masing-masing oleh Alice dan Bob
1. Alice  
$K\ =\ B^a\ mod\ p$

2. Bob  
$K\ =\ A^b\ mod\ p$

### Tambahan
Jika perhitungannya benar maka diperoleh nilai K yang sama antara Alice dan Bob.  
Nilai ini lah yang kemudian digunakan untuk melakukan proses enkripsi dan dekripsi  
seperti menggunakan algoritma Elgamal, AES, RSA, dan lain-lain.  
Pembuktian:  
1. Alice  
$K\ =\ B^a\ mod\ p$  
$K\ =\ (g^b)^a\ mod\ p$  
$K\ =\ g^{ab}\ mod\ p$  

2. Bob  
$K\ =\ A^b\ mod\ p$  
$K\ =\ (g^a)^b\ mod\ p$  
$K\ =\ g^{ab}\ mod\ p$

## Visualisasi
![DH-Scheme](https://i.stack.imgur.com/IPUgS.png)
