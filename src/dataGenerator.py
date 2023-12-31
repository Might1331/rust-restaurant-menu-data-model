import random
f = open("Onboarding_ProjectTQL/data.tql", "w")
f.write("insert\n\n\n")


placePool=[]
for i in range(0,1000):
    placePool.append("place"+str(i))
random.shuffle(placePool)

Prefix=["Grand","The","Old","Fast","Luigi's","Golden","La","100","Elite's","All"]
Middle=["Brunch","Indian","Chinese","Thai","Irish","London","Pizza","Patty","Burger","Blue","Hawaian","Japanese","Itiano","BBQ","Unlimited","Arabian","Veg","Non Vegas"]
Suffix=["Palace","House","Show","Delicacy","Garden","Tour","Cooks","Food","Eatarry","Cafe","Backery","Restaurant","Diner"]
menuPool=[]
random.shuffle(Prefix)
random.shuffle(Middle)
random.shuffle(Suffix)
for s1 in Prefix:
    for s2 in Middle:
        for s3 in Suffix:
            menuPool.append(s1+" "+s2)
            menuPool.append(s2+" "+s3)
            menuPool.append(s1+" "+s2)
            menuPool.append(s1+" "+s2+" "+s3)

random.shuffle(menuPool)
print(len(menuPool))
print(random.randint(0,1))
menus=[]
f.write("#menus\n\n")
for i in range(0,50):
    r1=random.Random
    is_veg="false"
    if(i%5):
        is_veg="true"
    menu=[placePool[i],menuPool[i],round(random.uniform(1,5),2),random.randint(111222555,999999999),is_veg]
    menus.append(menu)
    w="$m"+str(i)+" isa menu, has place \""+menu[0]+"\", has name \""+menu[1]+"\", has avg_rating "+str(menu[2])+", has call_number "+str(menu[3])+", has is_vegetarian "+menu[4]+";\n"
    f.write(w)
# $m1 isa menu, has place "place237", has name "Golden Arabian", has avg_rating 2.79, has call_number 578404695, has is_vegetarian false;

for i in range(0,2):
    print(menus[i])    


f.write("\n\n#raw_foods\n\n")
raw_foods=[]
rawPool=["fish","chicken","mutton","pwarn","cheese","onion","lentis","cabbage","spinach","flour","rice","milk","brocolli","beans","spice","msg","masala","salt","suger","souy sauce","red sauce","tomato sauce","tamarind sauce","beet roots"]
for i in range(0,500):
    name=""
    is_veg="true"
    c=i%len(rawPool)
    if(c<4):
        is_veg="false"
    raw_foods.append([rawPool[c]+str(i),is_veg])
    w="$rf"+str(i)+" isa raw_food,has name \""+raw_foods[i][0]+"\",has is_vegetarian "+raw_foods[i][1]+";\n"
    f.write(w)
# $rf1 isa raw_food,has name "pwarn4",has is_vegetarian false;

for i in range(0,2):
    print(raw_foods[i]) 
    
Prefix=["Roasted","Steamed","Fresh","Baked","Boiled","Fried","Organic","Spicy","Cheesy","Double","Smoking","Healthy","Green","Island","Hot","Sea","Red","Grand","The","Old","Luigi's","Golden","La"]
random.shuffle(Prefix)
dishes=[]
for p in Prefix:
    for i in range(0,20):
        idx=random.randint(0,len(raw_foods)-1)
        name=p+raw_foods[idx][0]
        is_ingredient={idx}
        is_veg="true"
        for i in range(0,random.randint(5,10)):
            idx=random.randint(0,len(raw_foods)-1)
            if(raw_foods[idx][1]=="false"):
                is_veg="false"
            is_ingredient.add(idx)
        speciality=set()
        ssz=random.randint(0,5)
        while(len(speciality)<ssz):
            idx=random.randint(0,len(menus)-1)
            if(menus[idx][4]!=is_veg):
                continue
            speciality.add(idx)
        sells=dict()
        for v in speciality:
            sells[v]=round(random.uniform(1,50),2)
        for i in range(0,random.randint(0,20)):
            idx=random.randint(0,len(menus)-1)
            sells[idx]=round(random.uniform(1,50),2)
            if(menus[idx][4]!=is_veg):
                continue
        dishes.append([name,sells,is_ingredient,speciality,is_veg])
random.shuffle(dishes)

f.write("\n\n#dishes\n\n")
scnt=0
icnt=0
spcnt=0
is_vegmenu=["true"]*50
for i in range(0,250):
    w="$d"+str(i)+" isa dish, has name \""+dishes[i][0]+"\", has is_vegetarian "+dishes[i][4]+";\n"
    f.write(w)
    for p in dishes[i][1]:
        w="$sl"+str(scnt)+" (seller: $m"+str(p)+",product: $d"+str(i)+") isa sells,has price "+str(dishes[i][1][p])+";\n"
        if(dishes[i][4]=="false"):
            is_vegmenu[p]="false"
        scnt+=1
        f.write(w)
    for p in dishes[i][2]:
        w="$is_i"+str(icnt)+" (dish: $d"+str(i)+",raw_food: $rf"+str(p)+") isa is_ingredient;\n"
        icnt+=1
        f.write(w)
    for p in dishes[i][3]:
        w="$sp"+str(spcnt)+" (restaurant: $m"+str(p)+",dish: $d"+str(i)+") isa speciality;\n"
        spcnt+=1
        f.write(w)
        
# $d0 isa dish, has name "Organicmutton1968", has is_vegetarian true;
# $sl0 (seller: $m0,product: $d0) isa sells,has price 19.51;
# $is_i0 (dish: $d0,raw_food: $rf0) isa is_ingredient;
# $sp0 (restaurant: $m0,dish: $d0) isa speciality;
for i in range(0,2):
    print(dishes[i]) 

f.close()