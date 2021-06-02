// Loads the dot file found at `dot_path` as text and displays it.
function loadSkillTree(dot_path) {
  var viz = new Viz();
  fetch(dot_path)
    .then(response => response.text())
    .then(text => {
      viz.renderSVGElement(text)
        .then(element => { document.body.appendChild(element); })
    });
}

function convertDivToSkillTree(divId, dotText) {
  new Viz().renderSVGElement(dotText.dot_text).then(svg_elem => {
    let parent = document.getElementById(divId);
    parent.appendChild(svg_elem);

    var element = svg_elem.children[0];
    panzoom(element, {
      bounds: true,
      boundsPadding: 0.1
    });
  })
}

for (let obj of SKILL_TREES) {
  convertDivToSkillTree(obj.id, obj.value);
}