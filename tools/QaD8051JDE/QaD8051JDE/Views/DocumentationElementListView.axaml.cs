using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;
using System;
using System.IO;
using System.Text.Json;
using System.Collections.Generic;
using System.Linq;
using Avalonia.Interactivity;
using Avalonia.Controls.ApplicationLifetimes;
using MessageBox.Avalonia.Enums;
using MessageBox.Avalonia.DTO;
using Avalonia.Threading;

namespace QaD8051JDE.Views
{
    public partial class DocumentationElementListView : UserControl
    {
        ViewModels.DocumentationElementListViewModel? ViewModel => DataContext as ViewModels.DocumentationElementListViewModel;

        public DocumentationElementListView()
        {
            InitializeComponent();
        }

        protected override void OnDataContextChanged(EventArgs e)
        {
            base.OnDataContextChanged(e);
            if (ViewModel is not null)
            {
                ViewModel.Saved += ViewModel_Saved;
            }
        }

        private void ViewModel_Saved(object? sender, EventArgs e)
        {
            if (_timer is not null)
                return;

            SavedText.IsVisible = true;
            _timer = new DispatcherTimer(
                new TimeSpan(0, 0, 3),
                DispatcherPriority.Background,
                (s, e) =>
            {
                SavedText.IsVisible = false;
                _timer?.Stop();
                _timer = null;
            });
            _timer.Start();
        }

        private DispatcherTimer? _timer;
    }
}
