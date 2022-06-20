# Prefetch Hash Cracker

![screenshot](https://user-images.githubusercontent.com/84273110/174433530-e43fa9c8-b779-4be1-9160-0536626b5ce3.jpg)

## Motivation
During the forensic analysis of a Windows machine, you may find evidence of a deleted prefetch file. Its content may not be recoverable, but the filename itself is often enough to find the full path of the executable for which the prefetch file was created.

## Using the tool
The following fields must be provided:
* *Executable name*  
  Including the extension. It will be embedded in the prefetch filename, unless [this](#The-29-character-limit) happens. 


* *Prefetch hash*  
  8 hexadecimal digits at the end of the prefetch filename, right before the `.pf` extension.  


* *Hash function*
* *Bodyfile*
* *Mount point*

### Hash function
There are 3 known prefetch hash functions:
* *SCCA XP*  
  Used in Windows XP
  
  
* *SCCA Vista*  
  Used in Windows Vista and Windows 10
  
  
* *SCCA 2008*  
  Used in Windows 7, Windows 8 and Windows 8.1


### Bodyfile
A bodyfile of the volume the executable was executed from.

The bodyfile format is not very restrictive, so there are a lot of variations of it - some of which are not supported. Body files created with `fls` and `MFTECmd` should work fine.

### Mount point
The mount point of the bodyfile, as underlined below:

<pre><code>0|<ins>C:</ins>/Users/Peter/Desktop ($FILE_NAME)|62694-48-2|d/d-wx-wx-wx|...</code></pre>

## How does it work?
The provided bodyfile is used to get the full path of every folder on the volume. The tool appends the provided executable name to each of those paths to create a possible full path for the executable. This possible full path is then hashed using the provided hash function. If the result matches the provided hash, the possible full path is outputted.

## Limitations
The following cases are not supported:
* Hosting applications, such as `svchost.exe` and `mmc.exe`
* Applications executed with the `/prefetch` flag
* Applications executed from a UNC (network) path

### The 29-character limit
If the executable name is longer than 29 characters (including the extension), it will be truncated in the prefetch filename. For example, executing this file from the `C:\Temp` directory:
```
This is a very long file nameSo this part will be truncated.exe
```
Will result in the creation of this prefetch file:
```
THIS IS A VERY LONG FILE NAME-D0B882CC.pf
```

In this case, the executable name cannot be derived from the prefetch filename, so you will not be able to provide it to the tool.

## License
[MIT](https://choosealicense.com/licenses/mit/)
