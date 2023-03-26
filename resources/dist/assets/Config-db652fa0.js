import{r as W,R as y,k as b,j as t,s as R,i as x,c as H,b as i,l as G,n as j,o as E,q,e as $,t as F,h as T,v as J,w as K,x as w,y as Q,u as X,C as Y,z as Z,B as ee,A as te,D as ne}from"./index-c48ccd7d.js";import{r as ae}from"./logs-9bbb50ca.js";import{S as _}from"./Select-ee82363c.js";function S(){return S=Object.assign||function(e){for(var s=1;s<arguments.length;s++){var r=arguments[s];for(var o in r)Object.prototype.hasOwnProperty.call(r,o)&&(e[o]=r[o])}return e},S.apply(this,arguments)}function se(e,s){if(e==null)return{};var r=oe(e,s),o,a;if(Object.getOwnPropertySymbols){var l=Object.getOwnPropertySymbols(e);for(a=0;a<l.length;a++)o=l[a],!(s.indexOf(o)>=0)&&Object.prototype.propertyIsEnumerable.call(e,o)&&(r[o]=e[o])}return r}function oe(e,s){if(e==null)return{};var r={},o=Object.keys(e),a,l;for(l=0;l<o.length;l++)a=o[l],!(s.indexOf(a)>=0)&&(r[a]=e[a]);return r}var O=W.forwardRef(function(e,s){var r=e.color,o=r===void 0?"currentColor":r,a=e.size,l=a===void 0?24:a,u=se(e,["color","size"]);return y.createElement("svg",S({ref:s,xmlns:"http://www.w3.org/2000/svg",width:l,height:l,viewBox:"0 0 24 24",fill:"none",stroke:o,strokeWidth:"2",strokeLinecap:"round",strokeLinejoin:"round"},u),y.createElement("path",{d:"M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"}),y.createElement("polyline",{points:"16 17 21 12 16 7"}),y.createElement("line",{x1:"21",y1:"12",x2:"9",y2:"12"}))});O.propTypes={color:b.string,size:b.oneOfType([b.string,b.number])};O.displayName="LogOut";const re=O,{useState:le,useRef:ce,useEffect:ie,useCallback:de}=x;function ue(e){return t("input",{className:R.input,...e})}function pe({value:e,...s}){const[r,o]=le(e),a=ce(e);ie(()=>{a.current!==e&&o(e),a.current=e},[e]);const l=de(u=>o(u.target.value),[o]);return t("input",{className:R.input,value:r,onChange:l,...s})}const he="_root_165du_1",ge="_section_165du_2",fe="_wrapSwitch_165du_26",me="_sep_165du_32",ve="_label_165du_45",c={root:he,section:ge,wrapSwitch:fe,sep:me,label:ve},ye="_fieldset_1ghjp_1",be="_input_1ghjp_9",Ce="_cnt_1ghjp_9",k={fieldset:ye,input:be,cnt:Ce};function we({OptionComponent:e,optionPropsList:s,selectedIndex:r,onChange:o}){const a=H("visually-hidden",k.input),l=u=>{o(u.target.value)};return t("fieldset",{className:k.fieldset,children:s.map((u,h)=>i("label",{children:[t("input",{type:"radio",checked:r===h,name:"selection",value:h,"aria-labelledby":"traffic chart type "+h,onChange:l,className:a}),t("div",{className:k.cnt,children:t(e,{...u})})]},h))})}const{useMemo:_e}=x,ke={plugins:{legend:{display:!1}},scales:{x:{display:!1,type:"category"},y:{display:!1,type:"linear"}}},M=[23e3,35e3,46e3,33e3,9e4,68e3,23e3,45e3],Se=[184e3,183e3,196e3,182e3,19e4,186e3,182e3,189e3],xe=M;function Oe({id:e}){const s=G.read(),r=_e(()=>({labels:xe,datasets:[{...j,...E[e].up,data:M},{...j,...E[e].down,data:Se}]}),[e]),o="chart-"+e;return q(s.Chart,o,r,null,ke),t("div",{style:{width:100,padding:5},children:t("canvas",{id:o})})}const{useEffect:z,useState:Ne,useCallback:f,useRef:Ie,useMemo:Le}=x,Pe=[{id:0},{id:1},{id:2},{id:3}],je=[["debug","Debug"],["info","Info"],["warning","Warning"],["error","Error"],["silent","Silent"]],Ee=[{key:"port",label:"HTTP Proxy Port"},{key:"socks-port",label:"SOCKS5 Proxy Port"},{key:"mixed-port",label:"Mixed Port"},{key:"redir-port",label:"Redir Port"}],Re=[["zh","中文"],["en","English"]],$e=[["Global","Global"],["Rule","Rule"],["Direct","Direct"]],Te=e=>({configs:F(e),apiConfig:T(e)}),Me=e=>({selectedChartStyleIndex:te(e),latencyTestUrl:ne(e),apiConfig:T(e)}),ze=$(Me)(Be),We=$(Te)(Ae);function Ae({dispatch:e,configs:s,apiConfig:r}){return z(()=>{e(J(r))},[e,r]),t(ze,{configs:s})}function Be({dispatch:e,configs:s,selectedChartStyleIndex:r,latencyTestUrl:o,apiConfig:a}){const[l,u]=Ne(s),h=Ie(s);z(()=>{h.current!==s&&u(s),h.current=s},[s]);const A=f(()=>{e(K("apiConfig"))},[e]),m=f((n,d)=>{u({...l,[n]:d})},[l]),B=f(n=>{const d="allow-lan",p=n;m(d,p),e(w(a,{"allow-lan":p}))},[a,e,m]),v=f(({name:n,value:d})=>{switch(n){case"mode":case"log-level":m(n,d),e(w(a,{[n]:d})),n==="log-level"&&ae({...a,logLevel:d});break;case"redir-port":case"socks-port":case"mixed-port":case"port":if(d!==""){const p=parseInt(d,10);if(p<0||p>65535)return}m(n,d);break;default:return}},[a,e,m]),U=f(n=>v(n.target),[v]),{selectChartStyleIndex:D,updateAppConfig:N}=Q(),I=f(n=>{const d=n.target,{name:p,value:P}=d;switch(p){case"port":case"socks-port":case"mixed-port":case"redir-port":{const C=parseInt(P,10);if(C<0||C>65535)return;e(w(a,{[p]:C}));break}case"latencyTestUrl":{N(p,P);break}default:throw new Error(`unknown input name ${p}`)}},[a,e,N]),V=Le(()=>{const n=l.mode;return typeof n=="string"&&n[0].toUpperCase()+n.slice(1)},[l.mode]),{t:g,i18n:L}=X();return i("div",{children:[t(Y,{title:g("Config")}),i("div",{className:c.root,children:[Ee.map(n=>l[n.key]!==void 0?i("div",{children:[t("div",{className:c.label,children:n.label}),t(ue,{name:n.key,value:l[n.key],onChange:U,onBlur:I})]},n.key):null),i("div",{children:[t("div",{className:c.label,children:"Mode"}),t(_,{options:$e,selected:V,onChange:n=>v({name:"mode",value:n.target.value})})]}),i("div",{children:[t("div",{className:c.label,children:"Log Level"}),t(_,{options:je,selected:l["log-level"],onChange:n=>v({name:"log-level",value:n.target.value})})]}),i("div",{children:[t("div",{className:c.label,children:"Allow LAN"}),t("div",{className:c.wrapSwitch,children:t(Z,{name:"allow-lan",checked:l["allow-lan"],onChange:B})})]})]}),t("div",{className:c.sep,children:t("div",{})}),i("div",{className:c.section,children:[i("div",{children:[t("div",{className:c.label,children:g("latency_test_url")}),t(pe,{name:"latencyTestUrl",type:"text",value:o,onBlur:I})]}),i("div",{children:[t("div",{className:c.label,children:g("lang")}),t("div",{children:t(_,{options:Re,selected:L.language,onChange:n=>L.changeLanguage(n.target.value)})})]}),i("div",{children:[t("div",{className:c.label,children:g("chart_style")}),t(we,{OptionComponent:Oe,optionPropsList:Pe,selectedIndex:r,onChange:D})]}),i("div",{children:[i("div",{className:c.label,children:[g("current_backend"),t("p",{children:a.baseURL})]}),t("div",{className:c.label,children:"Action"}),t(ee,{start:t(re,{size:16}),label:g("switch_backend"),onClick:A})]})]})]})}export{We as default};