// https://github.com/Faradey27/text-size/tree/master
var textSize=function(){function e(e,t){if(!o||!o.getContext)return console.error("Sorry your browser does not support canvas"),-1
var n=o.getContext("2d")
if(!n||!n.font||!n.measureText)return console.error("Sorry your browser does not support measureText method"),-1
n.font=t
var r=n.measureText(e)
return r&&r.width}function t(e,t){if(!document||!document.body)return console.error("Sorry DOM not ready"),0
r||(document.body.appendChild(n),r=!0),n.textContent=e,n.style.font=t,n.innerText=e,n.getBoundingClientRect||console.error("Sorry your browser does not support getBoundingClientRect method")
var o=n.getBoundingClientRect().width
return o}var o=document.createElement("canvas"),n=document.createElement("span")
n.style.position="fixed",n.style.visibility="hidden"
var r=!1,i=15,u="Arial",s="canvas",d="dom",a={getAvailableTypes:function(){return[s,d]},getTextWidth:function(o,n){if(!o.text)return 0
o.fontSize||console.error("You missed fontSize in config"),o.fontName||console.error("You missed fontName in config")
var r=(o.fontSize||i)+"px "+(o.fontName||u)
return"canvas"===n?e(o.text,r):t(o.text,r)}}
return a}()
"undefined"!=typeof module&&module.exports&&(module.exports=textSize)
