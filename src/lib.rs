#![feature(custom_attribute)]
#![feature(test)]
extern crate test;
use test::*;



pub fn for_every_pair<T>(rest:&mut [T],func: impl FnMut(&mut T,&mut T)){
	pub fn recc<T>(rest:&mut [T],mut func: impl FnMut(&mut T,&mut T)){
	    match rest.split_first_mut(){
	        Some((b1,rest))=>{
	            for b2 in rest.iter_mut(){
	                func(b1,b2);
	            } 
	            recc(rest,func);
	        },
	        None=>{

	        }
	    }
	}
	recc(rest,func);
}
pub fn for_every_pair_unsafe<T>(arr:&mut [T],mut func:impl FnMut(&mut T,&mut T)){
    unsafe{
        for x in 0..arr.len(){
            let xx=arr.get_unchecked_mut(x) as *mut T;
            for j in (x+1)..arr.len(){
                let j=arr.get_unchecked_mut(j);
                let xx=&mut*xx;
                func(xx,j);
            }
        }
    }
}

#[test]
fn test1(){
	let mut k=[0;100];
	for_every_pair(&mut k,|a,b|{
		*a+=1;
		*b+=1;
	});

	for b in k.iter(){
		assert!(*b==100-1);
	}
}

#[test]
fn test2(){
	let mut k=[0;100];
	for_every_pair_unsafe(&mut k,|a,b|{
		*a+=1;
		*b+=1;
	});

	for b in k.iter(){
		assert!(*b==100-1);
	}
}
#[bench]
fn bench1(bench:&mut Bencher){
	let mut k=[0;1000];
	bench.iter(||{
		for_every_pair(&mut k,|a,b|{
			*a+=1;
			*b+=1;
		})
	});


	black_box(k);
}

#[bench]
fn bench2(bench:&mut Bencher){
	let mut k=[0;1000];
	bench.iter(||{
		for_every_pair_unsafe(&mut k,|a,b|{
			*a+=1;
			*b+=1;
		})
	});


	black_box(k);
}

