//KHELFOUNE Amayas khelfouneamayas@gmail.com

utilisez std :: str ;

const UPPERCASE_OFFSET: i8  =  - 65 ;
const LOWERCASE_OFFSET: i8  =  26  -  97 ;
const NUM_OFFSET: i8  =  52  -  48 ;

fn  base64decode (entrée: String ) -> String {
    laissez result = input. trim (). chars ()
        . filter ( | & ch | ch ! =  '=' )                                
        . map ( | ch | {                                             
            soit ascii = ch comme  i8 ;                           
            laissez convertir =  correspondre à ch {
                '0' .. =  '9'  => ascii + NUM_OFFSET,
                'a' .. =  'z'  => ascii + LOWERCASE_OFFSET,
                'A' .. =  'Z'  => ascii + UPPERCASE_OFFSET,
                '+'  =>  62 ,
                '/'  =>  63
            };
            format! ( "{: # 08b}" , convertir) [ 2 ..]. to_string ()        
        })
        . collect :: < String > ()                                     
        . chars ()
        . collect :: < Vec < char >> ()
        . morceaux ( 8 )                                             
        . map ( | chunk | {
            laissez num_str = chunk. iter (). collect :: < String > ();
            usize :: from_str_radix ( & num_str, 2 ). unwrap () as  u8    
        })
        . collect :: < Vec <_>> ();
 
    laissez result =  str :: from_utf8 ( & result). dérouler ();              
    return  String :: from (résultat);
}
 
fn  main () {
    laissez input =  String :: new ();
    std :: io :: stdin (). read_line ( & mut entrée). dérouler ();
    println! ( "Entrée: {}" , entrée);
 
    laissez output =  base64decode (entrée);
    println! ( "Sortie: {}" , sortie);
}