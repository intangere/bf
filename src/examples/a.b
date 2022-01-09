,          read first char
>>,[>,]    have a 0 separator for the array then read all the other chars
<[->+<]    move the last input char to have a 0 separator
           Memory looks like A 0 B C D E F G H I J K L M N O P Q R S T U V W X Y 0 Z
           if input was the entire alphabet; and the cursor is just before Z
<[<]<.     Go to first char and print
>>[>]>.    Go to last char and print
<<[<]>     Go to array first value
[.>]       Iterate on the array : print and move
