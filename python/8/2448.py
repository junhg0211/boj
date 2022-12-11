s=int(input());l=' '*(s-1)+'*'+' '*s;print(l)
for i in range(s-1):
    if' * * 'in l:l=l.replace(' * * ','*****')
    else:
        l=l.replace('* ','**');t=list()
        for j in range(len(l)):
            if not t:t.append([l[j],1])
            else:
                if l[j]==t[-1][0]:t[-1][1]+=1
                else:t.append([l[j],1])
        l=''
        for t in t:
            if t[0]==' ':l+=' '*t[1]
            else:l=l[:-1]+'*'+' '*(t[1]-1)+'*'
    print(l)
