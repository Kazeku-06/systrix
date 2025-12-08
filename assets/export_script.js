// Systrix Export HTML JavaScript

// Sort table function
let sortDirection = {};

function sortTable(columnIndex) {
    const table = document.getElementById('processTable');
    const tbody = table.querySelector('tbody');
    const rows = Array.from(tbody.querySelectorAll('tr'));
    
    // Initialize sort direction for this column
    if (sortDirection[columnIndex] === undefined) {
        sortDirection[columnIndex] = true; // ascending
    } else {
        sortDirection[columnIndex] = !sortDirection[columnIndex];
    }
    
    const isAscending = sortDirection[columnIndex];
    
    rows.sort((a, b) => {
        const aValue = a.cells[columnIndex].textContent.trim();
        const bValue = b.cells[columnIndex].textContent.trim();
        
        // Try to parse as number
        const aNum = parseFloat(aValue.replace('%', ''));
        const bNum = parseFloat(bValue.replace('%', ''));
        
        if (!isNaN(aNum) && !isNaN(bNum)) {
            return isAscending ? aNum - bNum : bNum - aNum;
        }
        
        // String comparison
        return isAscending 
            ? aValue.localeCompare(bValue)
            : bValue.localeCompare(aValue);
    });
    
    // Clear tbody and append sorted rows
    tbody.innerHTML = '';
    rows.forEach(row => tbody.appendChild(row));
    
    // Update header indicators
    const headers = table.querySelectorAll('th');
    headers.forEach((header, index) => {
        if (index === columnIndex) {
            header.textContent = header.textContent.split(' ')[0] + (isAscending ? ' ▲' : ' ▼');
        } else {
            header.textContent = header.textContent.split(' ')[0] + ' ▼';
        }
    });
}

// Filter processes function
function filterProcesses() {
    const input = document.getElementById('processSearch');
    const filter = input.value.toUpperCase();
    const table = document.getElementById('processTable');
    const tbody = table.querySelector('tbody');
    const rows = tbody.getElementsByTagName('tr');
    
    let visibleCount = 0;
    
    for (let i = 0; i < rows.length; i++) {
        const cells = rows[i].getElementsByTagName('td');
        let found = false;
        
        // Search in PID, Name, User columns
        for (let j = 0; j < Math.min(3, cells.length); j++) {
            const cellText = cells[j].textContent || cells[j].innerText;
            if (cellText.toUpperCase().indexOf(filter) > -1) {
                found = true;
                break;
            }
        }
        
        if (found) {
            rows[i].style.display = '';
            visibleCount++;
        } else {
            rows[i].style.display = 'none';
        }
    }
    
    // Update process count
    const heading = document.querySelector('.processes-card h2');
    if (heading) {
        const totalCount = rows.length;
        if (filter) {
            heading.textContent = `📋 Top Processes (${visibleCount} of ${totalCount} shown)`;
        } else {
            heading.textContent = `📋 Top Processes (${totalCount} total)`;
        }
    }
}

// Add smooth scroll behavior
document.addEventListener('DOMContentLoaded', function() {
    // Smooth scroll for anchor links
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function (e) {
            e.preventDefault();
            const target = document.querySelector(this.getAttribute('href'));
            if (target) {
                target.scrollIntoView({
                    behavior: 'smooth',
                    block: 'start'
                });
            }
        });
    });
    
    // Add tooltips to progress bars
    document.querySelectorAll('.progress-bar').forEach(bar => {
        const fill = bar.querySelector('.progress-fill');
        if (fill) {
            const width = fill.style.width;
            bar.title = `Usage: ${width}`;
        }
    });
    
    // Highlight high usage items
    highlightHighUsage();
    
    // Add export buttons
    addExportButtons();
});

// Highlight high usage function
function highlightHighUsage() {
    document.querySelectorAll('.value').forEach(element => {
        const value = parseFloat(element.textContent);
        if (!isNaN(value)) {
            if (value > 90) {
                element.style.animation = 'pulse 2s infinite';
            }
        }
    });
}

// Add pulse animation for critical values
const style = document.createElement('style');
style.textContent = `
    @keyframes pulse {
        0%, 100% { opacity: 1; }
        50% { opacity: 0.7; }
    }
`;
document.head.appendChild(style);

// Add export buttons function
function addExportButtons() {
    const header = document.querySelector('header');
    if (header) {
        const buttonContainer = document.createElement('div');
        buttonContainer.style.marginTop = '20px';
        buttonContainer.innerHTML = `
            <button onclick="window.print()" style="
                background: white;
                color: #667eea;
                border: none;
                padding: 10px 20px;
                border-radius: 5px;
                cursor: pointer;
                font-size: 1em;
                font-weight: bold;
                margin: 0 5px;
                transition: all 0.3s ease;
            " onmouseover="this.style.background='#f8f9fa'" onmouseout="this.style.background='white'">
                🖨️ Print Report
            </button>
            <button onclick="copyToClipboard()" style="
                background: white;
                color: #667eea;
                border: none;
                padding: 10px 20px;
                border-radius: 5px;
                cursor: pointer;
                font-size: 1em;
                font-weight: bold;
                margin: 0 5px;
                transition: all 0.3s ease;
            " onmouseover="this.style.background='#f8f9fa'" onmouseout="this.style.background='white'">
                📋 Copy Summary
            </button>
        `;
        header.appendChild(buttonContainer);
    }
}

// Copy summary to clipboard
function copyToClipboard() {
    let summary = 'Systrix System Report\n';
    summary += '='.repeat(50) + '\n\n';
    
    // Get device name
    const deviceName = document.querySelector('.device-name');
    if (deviceName) {
        summary += deviceName.textContent + '\n';
    }
    
    // Get timestamp
    const timestamp = document.querySelector('.timestamp');
    if (timestamp) {
        summary += timestamp.textContent + '\n\n';
    }
    
    // Get metrics
    document.querySelectorAll('.card').forEach(card => {
        const title = card.querySelector('h2');
        const metric = card.querySelector('.metric');
        
        if (title && metric) {
            const label = metric.querySelector('.label');
            const value = metric.querySelector('.value');
            
            if (label && value) {
                summary += `${title.textContent}: ${value.textContent}\n`;
            }
        }
    });
    
    // Copy to clipboard
    navigator.clipboard.writeText(summary).then(() => {
        alert('Summary copied to clipboard!');
    }).catch(err => {
        console.error('Failed to copy:', err);
        alert('Failed to copy summary');
    });
}

// Add keyboard shortcuts
document.addEventListener('keydown', function(e) {
    // Ctrl+P or Cmd+P for print
    if ((e.ctrlKey || e.metaKey) && e.key === 'p') {
        e.preventDefault();
        window.print();
    }
    
    // Ctrl+F or Cmd+F for search
    if ((e.ctrlKey || e.metaKey) && e.key === 'f') {
        e.preventDefault();
        const searchInput = document.getElementById('processSearch');
        if (searchInput) {
            searchInput.focus();
            searchInput.select();
        }
    }
});

// Add refresh timestamp
setInterval(() => {
    const footer = document.querySelector('footer');
    if (footer) {
        const now = new Date();
        const viewTime = footer.querySelector('.view-time');
        if (!viewTime) {
            const p = document.createElement('p');
            p.className = 'view-time';
            p.style.fontSize = '0.9em';
            p.style.opacity = '0.8';
            p.textContent = `Viewing since: ${now.toLocaleString()}`;
            footer.appendChild(p);
        }
    }
}, 1000);

console.log('Systrix Report loaded successfully! 🚀');
console.log('Keyboard shortcuts:');
console.log('  Ctrl+P - Print report');
console.log('  Ctrl+F - Search processes');
