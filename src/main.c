#include <stdlib.h>
#include <stdio.h>

typedef __uint128_t set;

set pre[sizeof(set)*8];

set new_set(int start, int end){
    set buff = ((set)-1 << start) & ((set)1 << end+1)-1;
    return buff;
}

int forsen(set a, int turn){
    //printf("yep kok a: %x\n", a);
    if(a == 0){
        return turn & 1;
    }
    else if(turn & 1){
        for(int i = 0; a >> i; i++) if((a >> i) & 1) {
            if(!forsen(a & pre[i], turn+1)) return 0;
        }
        return 1;
    }
    else{
        for(int i = 0; a >> i; i++) if((a >> i) & 1){ 
            if(forsen(a & pre[i], turn+1)) return 1;
        }
        return 0;
    }
}

int main(){
    for(int i = 0; i < sizeof(set)*8; i++){
        set a = -1;
        for(int j = 1; j <= i; j++){
            if(i%j==0) a ^= (set)1 << j;
        }
        pre[i] = a;
    }
    //printf("fun: %x\n", pre[2]);
    for(int i = 1; i < 50; i++){
        set a = new_set(2, i);
        printf("%u,\n", pre[i]);
    }
}