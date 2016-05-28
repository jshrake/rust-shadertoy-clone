#define time iGlobalTime
float slope(float x, float m, float b) {
	return m*x + b;

}


float noise(vec2 uv) {
return fract(sin(dot(uv.xy ,vec2(532.1231,1378.3453))) * 53211.1223);
}

void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    float aspect = iResolution.x/iResolution.y;
	vec2 uv = fragCoord.xy / iResolution.xy;
    vec3 col = vec3(0.);
    vec3 topred_col = vec3(.96, 0.26, 0.25);
    vec3 bigpurp_col = vec3(0.58, .2, .72);

    vec3 midred_col = vec3(.96, 0.3, 0.38);
    vec3 botred_col = vec3(.95, .33, 0.4);
    vec3 bg_col = vec3(0.41, .17, .5);
    vec3 almost_bg_col = vec3(0.48, .16, .58);

   	vec3 cyan_col = vec3(.93, .78, 1.0);
	vec3 whiteish_col = vec3(.95, .84, 1.0);
    
    float topred_mover = cos(time)/30.-.02;
    float bigpurp_mover = cos(time)/100.-.02;

    float bigpurp =smoothstep(1., 1.0001, abs(uv.y-slope(uv.x, -2., .5+bigpurp_mover)));
    float topred =  smoothstep(1.01, 1.0, abs(uv.y - slope(uv.x, -5.2, .4+topred_mover)));
    float midred =  smoothstep(2.5, 2.49, abs(uv.y - slope(uv.x, 17., -1.8)));
    float botred =smoothstep(1.21, 1.2, abs(uv.y - slope(uv.x, 7., -.5)));
    float bg = smoothstep(1., 1.01, abs(uv.y-slope(uv.x, 2., 1.)));
    float cyan = smoothstep(2.001, 2., abs(uv.y-slope(uv.x, 3., 1.312)));
    float white = smoothstep(2.008, 2., abs(uv.y-slope(uv.x, 2.3, 1.49)));
    float almost_bg = smoothstep(2.01, 2., abs(uv.y-slope(uv.x, 1.8, 1.61)));
    
    float t = .0026;
    float h = .013;
    float y = .923*aspect;
    float line =  step(length(uv.y - .13+t), t*aspect);
    float line2 = step(length(uv.x -.923), t);
/*
    float hz =.908;
    float vz = .1;
    if (uv.x > hz && uv.x < hz+.03 && uv.y > vz && uv.y < vz+.03*aspect)
    	col -= max(line, line2)*vec3(1., 0., .0);

    float circle = smoothstep(.1, .09, length(vec2(uv.x*aspect, uv.y) - vec2(.923*aspect, .13)));
	float circle_shadow = smoothstep(.12, .07, length(vec2(uv.x*aspect, uv.y) - vec2(.923*aspect, .121)))/6.;
	*/
    float bigpurp_shadow = smoothstep(.14+bigpurp_mover, .0, abs(uv.y-slope(uv.x, -2., 1.49-bigpurp_mover)))/13.;
	float topred_shadow = smoothstep(.2-topred_mover, .0, abs(uv.y-slope(uv.x, -5.2,1.3-topred_mover)))/2.;
	float midred_shadow = smoothstep(.06, .0, abs(uv.y-slope(uv.x, 17.,-4.32)))/3.;
	float botred_shadow = smoothstep(.04, .0, abs(uv.y-slope(uv.x, 7.,-1.7)))/3.;
	float cyan_shadow = smoothstep(.01, .0, abs(uv.y-slope(uv.x, 3.,-.699)))/3.;
	float almost_bg_shadow = smoothstep(.02, .0, abs(uv.y-slope(uv.x, 1.8,-.39)))/3.;

   
    col += bigpurp_col * bigpurp;
    col += topred_col * topred;
        float f = .5;

    //col += vec3(1., -.1, -.5)*circle;                          	
	//if (circle < f)
      //  col -= circle_shadow;

    if (bigpurp < f && topred < f) {
        
        col -= topred_shadow;
        col -= bigpurp_shadow;
        col += midred_col * midred;
        if (midred < f) {
            
            col -= midred_shadow;
            col -= bigpurp_shadow;
            col += botred_col * botred;
            if (botred < f) {
                col-=botred_shadow;
                col += cyan_col * cyan;
            	if (cyan < f) {
                    col -= cyan_shadow;
                    col += whiteish_col * white;
                    if (white < f){
                        col += almost_bg_col * almost_bg;
                        if (almost_bg < f) {
                        	col -= almost_bg_shadow;
                			col += bg_col * bg;
                        }
                    }
                }
            }
        }
    }
        	col += noise(uv)/18.;

    fragColor = vec4(col,1.0);
}
