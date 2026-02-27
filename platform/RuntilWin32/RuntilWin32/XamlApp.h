#pragma once

#include "framework.h"
#include "MddBootstrap.h"

using namespace winrt;
using namespace Microsoft::UI::Xaml;
using namespace Microsoft::UI::Xaml::Markup;

struct MyApp : ApplicationT<MyApp, IXamlMetadataProvider>
{
    std::vector<IXamlMetadataProvider> m_providers;

    std::vector<IXamlMetadataProvider> GetProviders() {
        if (m_providers.empty()) {
            m_providers.push_back(XamlTypeInfo::XamlControlsXamlMetaDataProvider());
        }
        return m_providers;
    }

    MyApp() {
        Hosting::WindowsXamlManager::InitializeForCurrentThread();

        auto xamlResources = Controls::XamlControlsResources();

        Resources().MergedDictionaries().Append(xamlResources);
    }

    void OnLaunched(LaunchActivatedEventArgs const&)
    {
        auto window = Window();
        window.ExtendsContentIntoTitleBar(true);
        Controls::Button button;
        button.Content(box_value(L"Hello World!"));
        window.Content(button);
        Media::MicaBackdrop mica;
        window.SystemBackdrop(mica);
        window.Activate();
    }

    IXamlType GetXamlType(Windows::UI::Xaml::Interop::TypeName const& type) {
        auto providers = GetProviders();
        for (const auto& provider : providers) {
            if (auto result = provider.GetXamlType(type)) {
                return result;
            }
        }
        return nullptr;
    }
    IXamlType GetXamlType(hstring const& name) {
        auto providers = GetProviders();
        for (const auto& provider : providers) {
            if (auto result = provider.GetXamlType(name)) {
                return result;
            }
        }
        return nullptr;
    }
    com_array<XmlnsDefinition> GetXmlnsDefinitions() {
        auto providers = GetProviders();
        std::vector<::winrt::Microsoft::UI::Xaml::Markup::XmlnsDefinition> allDefinitions;
        for (const auto& provider : providers) {
            const auto& definitions = provider.GetXmlnsDefinitions();
            allDefinitions.insert(allDefinitions.cend(), definitions.cbegin(), definitions.cend());
        }
        return winrt::com_array<::winrt::Microsoft::UI::Xaml::Markup::XmlnsDefinition>(
            allDefinitions.begin(), allDefinitions.end());
    }
};